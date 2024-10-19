use core::ffi::{c_void, CStr};

use anyhow::anyhow;

use crate::sys;

const GT911_DEV_PATH: &CStr = c"/dev/input0";
// const GT911_ADDR: u8 = 0x14;
const GT911_ADDR: u8 = 0x5D;
const GT911_I2C: i32 = 1;

// vtable shit let's goooooooooo
static GT911_CONFIG: sys::gt9xx_board_s = sys::gt9xx_board_s {
    irq_attach: Some(irq_attach),
    irq_enable: Some(irq_enable),
    set_power: Some(set_power),
};

unsafe extern "C" fn irq_attach(
    _state: *const sys::gt9xx_board_s,
    _isr: Option<unsafe extern "C" fn(i32, *mut c_void, *mut c_void) -> i32>,
    _arg: *mut c_void,
) -> i32 {
    sys::puts(c"GT911 Attach!\n".as_ptr());
    0
}

unsafe extern "C" fn irq_enable(_state: *const sys::gt9xx_board_s, enable: bool) {
    if enable {
        sys::puts(c"GT911 Enable!\n".as_ptr());
    } else {
        sys::puts(c"GT911 Disable!\n".as_ptr());
    }
}

unsafe extern "C" fn set_power(_state: *const sys::gt9xx_board_s, on: bool) -> i32 {
    if on {
        sys::puts(c"GT911 On!\n".as_ptr());
    } else {
        sys::puts(c"GT911 Off!\n".as_ptr());
    }
    0
}
pub fn register_gt911() -> anyhow::Result<()> {
    unsafe { inner_register_gt911() }
}
pub unsafe fn inner_register_gt911() -> anyhow::Result<()> {
    let i2c = sys::imxrt_i2cbus_initialize(GT911_I2C);
    if i2c.is_null() {
        return Err(anyhow!("Cannot get I2C"));
    }
    let err = sys::gt9xx_register(
        GT911_DEV_PATH.as_ptr(),
        i2c,
        GT911_ADDR,
        &GT911_CONFIG as *const sys::gt9xx_board_s,
    );
    if err < 0 {
        let err = sys::imxrt_i2cbus_uninitialize(i2c);
        debug_assert!(err >= 0);
        return Err(anyhow!("Cannot register GT911"));
    }
    Ok(())
}
