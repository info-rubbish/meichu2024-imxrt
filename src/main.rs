#![no_std]
#![no_main]

use core::ffi::c_void;

mod sys;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[global_allocator]
static GLOBAL: CAllocator = CAllocator;

struct CAllocator;

unsafe impl core::alloc::GlobalAlloc for CAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        sys::aligned_alloc(layout.align() as u32, layout.size() as u32) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        sys::free(ptr as *mut c_void);
    }

    unsafe fn realloc(
        &self,
        ptr: *mut u8,
        _layout: core::alloc::Layout,
        new_size: usize,
    ) -> *mut u8 {
        sys::realloc(ptr as *mut c_void, new_size as u32) as *mut u8
    }
}

extern crate alloc;

extern "C" fn rust_main() {
    let v = alloc::vec![1, 2, 3];
    
}
