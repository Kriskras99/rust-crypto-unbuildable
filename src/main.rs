#![no_main]
#![no_std]

use core::panic::PanicInfo;
use chacha20poly1305;

#[panic_handler]
pub fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

pub extern "C" fn main() -> ! {
    loop {}
}
