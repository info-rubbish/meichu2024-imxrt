use crate::sys;

const C2S_SIZE: usize = 5;
const S2C_SIZE: usize = 12;
const SECRET: u8 = 0b10101010;

pub fn read(command: [u8; C2S_SIZE]) -> [u8; S2C_SIZE] {
    for byte in command {
        unsafe {
            sys::putchar((byte ^ SECRET) as i32);
        }
    }
    unsafe {
        sys::fflush(sys::fdopen(sys::STDOUT_FILENO as i32, c"w".as_ptr()));
    }

    let mut buffer = [0; S2C_SIZE];
    for byte in &mut buffer {
        *byte = unsafe { sys::getchar() as u8 } ^ SECRET;
    }
    buffer
}
