#![no_std]
#![no_main]

extern crate alloc;

use core::{ffi::c_void, u32};

use alloc::{ffi::CString, string::ToString};

mod fs;
mod gt911;
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
    fn nsh_consolemain(argc: i32, argv: *const *const u8) -> i32;
}

#[no_mangle]
fn nxp_main(argc: i32, argv: *const *const u8) -> i32 {
    unsafe {
        nsh_initialize();
        sys::puts(c"NXP Start\n".as_ptr());
    }
    unsafe {
        sys::puts(c"Add Driver\n".as_ptr());
    }
    gt911::register_gt911().unwrap();

    unsafe {
        sys::puts(c"Run UI\n".as_ptr());
    }
    ui::run_ui().unwrap();
    // return back to nsh
    // sys::task_create(name, priority, stack_size, entry, argv)
    unsafe {
        nsh_consolemain(argc, argv);
        // nsh_main(argc, argv);
    }
    halt()
}

fn halt() -> ! {
    loop {
        unsafe {
            sys::sleep(1000);
        }
    }
}
