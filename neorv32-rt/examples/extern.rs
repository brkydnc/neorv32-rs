#![no_std]
#![no_main]

use core::panic::PanicInfo;
use neorv32_rt as _;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main() -> ! {
    loop {}
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
