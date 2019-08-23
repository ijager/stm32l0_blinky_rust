// std and main are not available for bare metal software
#![no_std]
#![no_main]

extern crate panic_halt;
extern crate cortex_m_rt;

use cortex_m_rt::entry;

use stm32l0::stm32l0x2;
use cortex_m::asm;


// use `main` as the entry point of this application
#[entry]
fn main() -> ! {
    // get handles to the hardware

    let peripherals = stm32l0x2::Peripherals::take().unwrap();
    let gpioa = &peripherals.GPIOA;
    let gpiob = &peripherals.GPIOB;

    let rcc = &peripherals.RCC;

    // enable clocks for GPIOB and GPIOA peripherals
    rcc.iopenr.write(|w| w.iopben().set_bit().iopaen().set_bit());

    unsafe {
        // SVD made this unsafe without reason...
        // set pin dir
        gpioa.moder.write(|w| w.mode5().bits(1));
        gpiob.moder.write(|w| w.mode5().bits(1));
    }
    loop {

        // Multiple ways to blink a led

        gpiob.bsrr.write(|w| w.bs5().set_bit());
        cortex_m::asm::delay(2000000);
        gpiob.bsrr.write(|w| w.br5().set_bit());
        cortex_m::asm::delay(2000000);

        gpioa.odr.write(|w| {
            w.od5().bit(gpioa.odr.read().od5().bit_is_clear())
        });

        for _i in 0..1000 {
            asm::nop()
        }
    }
}
