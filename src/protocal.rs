use crate::sys;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Signal {
    Start = 0xf7,
    Req = 0xf8,
}

const C2S_SIZE: usize = 5;
const S2C_SIZE: usize = 12;

pub fn read_server_signal() -> Option<Signal> {
    let signal = unsafe { sys::getchar() };
    match signal {
        0xf7 => Some(Signal::Start),
        0xf8 => Some(Signal::Req),
        _ => None,
    }
}

pub fn wait_signal(signal: Signal) {
    loop {
        if Some(signal) == read_server_signal() {
            break;
        }
    }
}

pub fn wait_connect() {
    wait_signal(Signal::Start);
    unsafe {
        sys::putchar(Signal::Start as _);
    }
}

pub fn read(command: [u8; C2S_SIZE]) -> [u8; S2C_SIZE] {
    unsafe {
        sys::putchar(Signal::Req as _);
    }
    for byte in command {
        unsafe {
            sys::putchar(Signal::Req as _);
        }
    }
    wait_signal(Signal::Req);
    let mut buffer = [0; S2C_SIZE];
    for byte in &mut buffer {
        *byte = unsafe { sys::getchar() as u8 };
    }
    buffer
}
