//#![deny(unsafe_code)]
//#![deny(warnings)]
#![no_std]
#![no_main]

extern crate cortex_m_rt as rt;
extern crate cortex_m;
extern crate f3;
extern crate panic_semihosting;

use f3::hal::delay::Delay;
use f3::hal::prelude::*;
use f3::hal::stm32f30x;
use f3::led::{Leds, Led};

use cortex_m_semihosting::hprintln;
use cortex_m_rt::entry;


#[entry]
fn main() -> ! {
    hprintln!("Start Lesdsdd!").unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let gpioe = dp.GPIOE.split(&mut rcc.ahb);

    // clock configuration using the default settings (all clocks run at 8 MHz)
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    // TRY this alternate clock configuration (all clocks run at 16 MHz)
    // let clocks = rcc.cfgr.sysclk(16.mhz()).freeze(&mut flash.acr);

    let mut leds = Leds::new(gpioe);
    let mut delay = Delay::new(cp.SYST, clocks);


    let n = leds.len();
    hprintln!("Start Loop!").unwrap();
    loop {
        for curr in 0..n {
            let next = (curr + 1) % n;
            leds[curr].off();
            leds[next].on();

            delay.delay_ms(100_u8);
        }
    }
}
