#[allow(warnings)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/nuttx.rs"));
}

pub use bindings::*;
