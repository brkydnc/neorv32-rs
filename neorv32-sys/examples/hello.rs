#![no_main]
#![no_std]

use neorv32_rt as _;
use panic_halt as _;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main() -> ! {
    loop {}
}
