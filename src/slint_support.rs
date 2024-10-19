use crate::{fs, sys, time};
use alloc::{boxed::Box, rc::Rc, string::ToString, vec::Vec};
use anyhow::{anyhow, Result};
use core::{ffi::CStr, mem::MaybeUninit, time::Duration};
use slint::{
    platform::{
        software_renderer::{MinimalSoftwareWindow, RepaintBufferType, Rgb565Pixel},
        update_timers_and_animations, Platform, PointerEventButton, WindowAdapter, WindowEvent,
    },
    LogicalPosition, PhysicalPosition, PhysicalSize, PlatformError,
};

const FB_PATH: &CStr = c"/dev/fb0";

pub struct MIXRT {
    clock: time::Clock,
    begin_ts: Duration,
    window: Rc<MinimalSoftwareWindow>,
    fb_file: fs::File,
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

        let clock = time::Clock::new(sys::CLOCK_REALTIME as i32);
        let begin_ts = clock.get()?;

        let input = fs::File::open(c"/dev/input0", sys::O_RDONLY as i32)?;

        Ok(Self {
            clock,
            begin_ts,
            window,
            fb_file,
            panel_info,
            input,
        })
    }
}

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
        loop {
            update_timers_and_animations();
            self.window.draw_if_needed(|renderer| {
                let buffer = unsafe {
                    core::slice::from_raw_parts_mut(
                        self.panel_info.fbmem as *mut Rgb565Pixel,
                        self.panel_info.fblen / size_of::<Rgb565Pixel>(),
                    )
                };
                renderer.render(
                    buffer,
                    self.panel_info.stride as usize / size_of::<Rgb565Pixel>(),
                );
            });

            let input = unsafe { self.input.read::<sys::touch_sample_s>() }
                .map_err(|e| PlatformError::Other(e.to_string()))?;
            handle_touch(&self.window, input);

            if !self.window.has_active_animations() {
                unsafe {
                    sys::usleep(50);
                }
            }
        }
    }
}

fn handle_touch(window: &Rc<MinimalSoftwareWindow>, input: sys::touch_sample_s) {
    if input.npoints == 0 {
        window.dispatch_event(WindowEvent::PointerExited);
        return;
    }
    let button = PointerEventButton::Left;
    let position =
        PhysicalPosition::new(input.point[0].x as _, input.point[0].y as _).to_logical(1.0);

    let flag = input.point[0].flags as u32;
    if flag & sys::TOUCH_MOVE != 0 {
        window.dispatch_event(WindowEvent::PointerMoved { position });
        return;
    }
    if flag & sys::TOUCH_DOWN != 0 {
        window.dispatch_event(WindowEvent::PointerPressed { position, button });
    }
    if flag & sys::TOUCH_UP != 0 {
        window.dispatch_event(WindowEvent::PointerReleased { position, button });
    }
}
