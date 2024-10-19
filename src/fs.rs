use core::{
    ffi::{c_void, CStr},
    mem::MaybeUninit,
};

use anyhow::{anyhow, Result};

use crate::sys;

pub struct File(i32);

impl File {
    pub fn open(path: &CStr, oflag: i32) -> Result<Self> {
        unsafe {
            let fd = sys::open(path.as_ptr(), oflag);
            if fd < 0 {
                return Err(anyhow!("Cannot open file"));
            }
            Ok(Self(fd))
        }
    }

    pub fn close(self) -> Result<()> {
        let err = unsafe { sys::close(self.0) };
        if err < 0 {
            return Err(anyhow!("Cannot close file: {err}"));
        }
        Ok(())
    }

    /// caller must provide a valid `T` for req
    pub unsafe fn ioctl<T>(&self, req: i32) -> Result<T> {
        let mut res = MaybeUninit::<T>::uninit();
        let err = sys::ioctl(self.0, req, res.as_mut_ptr());
        if err < 0 {
            return Err(anyhow!("Ioctl fail: {err}"));
        }
        Ok(res.assume_init())
    }

    pub unsafe fn read<T>(&self) -> Result<T> {
        let mut buf = MaybeUninit::<T>::uninit();
        let err = unsafe { sys::read(self.0, buf.as_mut_ptr() as *mut c_void, size_of::<T>()) };
        if err < 0 {
            return Err(anyhow!("Read fail: {err}"));
        }
        Ok(unsafe { buf.assume_init() })
    }

    pub fn fd(&self) -> i32 {
        self.0
    }
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe {
            let err = sys::close(self.0);
            debug_assert_eq!(err, 0);
        }
    }
}
