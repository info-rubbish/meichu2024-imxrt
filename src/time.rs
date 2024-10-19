use core::{ffi::c_void, mem::MaybeUninit, ptr, time::Duration};

use crate::sys;
use anyhow::{anyhow, Result};

pub struct Clock(sys::clockid_t);

impl Clock {
    pub fn new(id: sys::clockid_t) -> Self {
        Self(id)
    }

    pub fn get(&self) -> Result<Duration> {
        let mut tp = MaybeUninit::uninit();
        let err = unsafe { sys::clock_gettime(self.0, tp.as_mut_ptr()) };
        if err < 0 {
            return Err(anyhow!("Cannot get clock"));
        }
        let tp = unsafe { tp.assume_init() };
        Ok(Duration::new(tp.tv_sec as u64, tp.tv_nsec as u32))
    }

    pub fn set(&self, tp: Duration) -> Result<()> {
        let tp = sys::timespec {
            tv_sec: tp.as_secs() as u32,
            tv_nsec: tp.subsec_nanos() as i32,
        };
        let err = unsafe { sys::clock_settime(self.0, &tp) };
        if err < 0 {
            return Err(anyhow!("Cannot set clock"));
        }
        Ok(())
    }
}
