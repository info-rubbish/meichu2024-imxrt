#[allow(warnings)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/nuttx.rs"));
}

pub use bindings::*;

/// Get color plane info
/// Argument: writable struct
/// `fb_videoinfo_s`
pub const FBIOGET_VIDEOINFO: u32 = _FBIOCBASE | 0x0001;

/// Get video plane info
/// Argument: writable struct
/// `fb_planeinfo_s`
pub const FBIOGET_PLANEINFO: u32 = _FBIOCBASE | 0x0002;
