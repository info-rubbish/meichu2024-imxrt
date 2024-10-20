use crate::{fs, sys, time};
use alloc::{rc::Rc, string::ToString};
use anyhow::Result;
use core::{ffi::CStr, time::Duration};
use slint::{
    platform::{
        software_renderer::{
            LineBufferProvider, MinimalSoftwareWindow, RepaintBufferType, Rgb565Pixel,
        },
        update_timers_and_animations, PointerEventButton, WindowEvent,
    },
    PhysicalPosition, PhysicalSize, PlatformError,
};

const FB_PATH: &CStr = c"/dev/fb0";

pub struct MIXRT {
    clock: time::Clock,
    begin_ts: Duration,
    window: Rc<MinimalSoftwareWindow>,
    _fb_file: fs::File,
    panel_info: sys::fb_planeinfo_s,
    input: fs::File,
}

impl MIXRT {
    pub fn new() -> Result<Self> {
        let window = MinimalSoftwareWindow::new(RepaintBufferType::ReusedBuffer);
        let fb_file = fs::File::open(FB_PATH, sys::O_RDWR as i32)?;

        let video_info: sys::fb_videoinfo_s =
            unsafe { fb_file.ioctl(sys::FBIOGET_VIDEOINFO as i32)? };
        window.set_size(PhysicalSize::new(
            video_info.xres as u32,
            video_info.yres as u32,
        ));

        let panel_info: sys::fb_planeinfo_s =
            unsafe { fb_file.ioctl(sys::FBIOGET_PLANEINFO as i32)? };

        // set default color
        unsafe {
            sys::memset(panel_info.fbmem, 0, panel_info.fblen as u32);
        }

        let clock = time::Clock::new(sys::CLOCK_MONOTONIC as i32);
        let begin_ts = clock.get()?;

        let input = fs::File::open(c"/dev/input0", sys::O_RDONLY as i32)?;

        Ok(Self {
            clock,
            begin_ts,
            window,
            _fb_file: fb_file,
            panel_info,
            input,
        })
    }
}

const FRAME_RATE: u32 = 40;

impl slint::platform::Platform for MIXRT {
    fn create_window_adapter(
        &self,
    ) -> Result<alloc::rc::Rc<dyn slint::platform::WindowAdapter>, slint::PlatformError> {
        Ok(self.window.clone())
    }

    fn duration_since_start(&self) -> core::time::Duration {
        self.clock.get().unwrap() - self.begin_ts
    }

    fn run_event_loop(&self) -> Result<(), slint::PlatformError> {
        let mut touch = Touch(None);
        let mmap = unsafe {
            core::slice::from_raw_parts_mut(
                self.panel_info.fbmem as *mut Rgb565Pixel,
                self.panel_info.fblen / size_of::<Rgb565Pixel>(),
            )
        };
        loop {
            update_timers_and_animations();
            let buffer = MMapLineBuffer {
                stride: self.panel_info.stride as usize / size_of::<Rgb565Pixel>(),
                mmap,
            };
            self.window.draw_if_needed(|renderer| {
                renderer.render_by_line(buffer);
            });

            let input = unsafe { self.input.read::<sys::touch_sample_s>() }
                .map_err(|e| PlatformError::Other(e.to_string()))?;
            touch.handle_touch(&self.window, input, self.duration_since_start());

            if !self.window.has_active_animations() {
                unsafe {
                    sys::usleep(1000 / FRAME_RATE);
                }
            }
        }
    }
}

const TOUCH_SCALE: f32 = 1.0;
const TOUCH_MOVE_THRESHOLD: u32 = 20;
const TOUCH_MOVE_INTERVAL: Duration = Duration::from_millis(300);

struct Touch(Option<TouchSample>);

#[derive(Debug, Clone, Copy)]
struct TouchSample {
    position: PhysicalPosition,
    timestamp: Duration,
}

impl Touch {
    fn handle_touch(
        &mut self,
        window: &Rc<MinimalSoftwareWindow>,
        input: sys::touch_sample_s,
        now_time: Duration,
    ) {
        let button = PointerEventButton::Left;

        if input.npoints == 0 {
            if let Some(touch) = self.0 {
                if now_time - touch.timestamp > TOUCH_MOVE_INTERVAL {
                    self.0 = None;
                    window.dispatch_event(WindowEvent::PointerReleased {
                        position: touch.position.to_logical(TOUCH_SCALE),
                        button,
                    });
                    window.dispatch_event(WindowEvent::PointerExited);
                }
            }
            return;
        }

        let physical_position = PhysicalPosition::new(input.point[0].x as _, input.point[0].y as _);
        let position = physical_position.to_logical(TOUCH_SCALE);

        let flag = input.point[0].flags as u32;

        if flag & sys::TOUCH_UP != 0 {
            self.0 = Some(TouchSample {
                position: physical_position,
                timestamp: now_time,
            });
        }

        if flag & sys::TOUCH_DOWN != 0 {
            if let Some(touch) = self.0.take() {
                if touch.position.x.abs_diff(physical_position.x) < TOUCH_MOVE_THRESHOLD
                    && touch.position.y.abs_diff(physical_position.y) < TOUCH_MOVE_THRESHOLD
                {
                    return;
                }
                window.dispatch_event(WindowEvent::PointerReleased {
                    position: touch.position.to_logical(TOUCH_SCALE),
                    button,
                });
            }

            window.dispatch_event(WindowEvent::PointerPressed { position, button });
        }
    }
}

#[derive(Debug)]
struct MMapLineBuffer<'a> {
    stride: usize,
    mmap: &'a mut [Rgb565Pixel],
}

impl<'a> LineBufferProvider for MMapLineBuffer<'a> {
    type TargetPixel = Rgb565Pixel;

    fn process_line(
        &mut self,
        line: usize,
        range: core::ops::Range<usize>,
        render_fn: impl FnOnce(&mut [Self::TargetPixel]),
    ) {
        render_fn(&mut self.mmap[line * self.stride..][range]);
    }
}
