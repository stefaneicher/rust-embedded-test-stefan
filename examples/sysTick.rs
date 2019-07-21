//https://rust-embedded.github.io/book/start/registers.html#starting-at-the-bottom


#![no_main]
#![no_std]

// dev profile: easier to debug panics; can put a breakpoint on `rust_begin_unwind`
#[cfg(debug_assertions)]
extern crate panic_halt;

// release profile: minimize the binary size of the application
#[cfg(not(debug_assertions))]
extern crate panic_abort;

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};

use cortex_m::peripheral::{syst, Peripherals};

#[entry]
fn main() -> ! {
    hprintln!("Start").unwrap();
    let mut peripherals = Peripherals::take().unwrap();
    let mut systick = peripherals.SYST;
    systick.set_clock_source(syst::SystClkSource::Core);
    systick.set_reload(1_0000);
    systick.clear_current();
    systick.enable_counter();
    hprintln!("setup systick").unwrap();
    while !systick.has_wrapped() {
        // Loop

    }

    hprintln!("systick wraped").unwrap();
    loop {
        hprintln!("loop").unwrap();
    }
}
