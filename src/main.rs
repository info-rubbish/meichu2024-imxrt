#![no_std]
#![no_main]

extern crate alloc;

use core::{ffi::c_void, ptr, u32};

use alloc::{ffi::CString, string::ToString};
use sys::sleep;

mod backend;
mod fs;
mod gt911;
mod protocal;
mod slint_support;
mod sys;
mod time;
mod ui;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let err_msg = CString::new(info.to_string());
    unsafe {
        sys::puts(err_msg.unwrap_unchecked().as_ptr());
        halt()
    }
}

#[global_allocator]
static GLOBAL: CAllocator = CAllocator;

struct CAllocator;

unsafe impl core::alloc::GlobalAlloc for CAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let align = layout.align().max(core::mem::size_of::<usize>()) as u32;
        let size = layout.size() as u32;

        sys::memalign(align, size) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        sys::free(ptr as *mut c_void);
    }

    // unsafe fn realloc(
    //     &self,
    //     ptr: *mut u8,
    //     _layout: core::alloc::Layout,
    //     new_size: usize,
    // ) -> *mut u8 {
    //     sys::realloc(ptr as *mut c_void, new_size as u32) as *mut u8
    // }
}

extern "C" {
    fn nsh_initialize();
    fn nsh_consolemain(argc: i32, argv: *mut *mut i8) -> i32;
}

#[no_mangle]
fn nxp_main(argc: i32, argv: *const *const u8) -> i32 {
    unsafe {
        nsh_initialize();
    }
    gt911::register_gt911().unwrap();

    // spawn nsh
    // unsafe {
    //     sys::task_create(
    //         c"nsh".as_ptr(),
    //         100,
    //         2048,
    //         Some(nsh_consolemain),
    //         ptr::null(),
    //     );
    // }

    ui::run_ui().unwrap();
    halt()
}

fn halt() -> ! {
    loop {
        unsafe {
            sys::sleep(1000);
        }
    }
}
