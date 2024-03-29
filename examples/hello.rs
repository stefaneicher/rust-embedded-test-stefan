//! Prints "Hello, world!" on the host console using semihosting

#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m_semihosting::hprintln;
use cortex_m::peripheral::{syst, Peripherals};
use cortex_m_rt::entry;


#[entry]
fn main() -> ! {
    hprintln!("Hello, world!").unwrap();
    let mut peripherals = Peripherals::take().unwrap();
    let mut systick = peripherals.SYST;
    systick.set_clock_source(syst::SystClkSource::Core);
    systick.set_reload(1_000);
    systick.clear_current();
    systick.enable_counter();
    hprintln!("SysTick enabled!").unwrap();
    while !systick.has_wrapped() {
        // Loop<<
    }

    loop {}
}