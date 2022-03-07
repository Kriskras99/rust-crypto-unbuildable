#![no_main]
#![no_std]

use core::panic::PanicInfo;
use heapless::Vec; // fixed capacity `std::Vec`

#[panic_handler]
pub fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

pub extern "C" fn main() -> ! {
    // on the stack
    let mut xs: Vec<u8, 8> = Vec::new(); // can hold up to 8 elements
    xs.push(42).unwrap();
    assert_eq!(xs.pop(), Some(42));

    loop {}
}
