// \file cortex_m4.rs
// Copyright (c) 2023 Infineon Technologies AG
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation
// files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy,
// modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
// OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE
// ---

use cortex_m::delay::Delay;
use cortex_m_semihosting::hprintln;

use cyt2b7 as pac;
use pac::gpio as GPIO;

/// Executes before the main function and can be used for HW initialization
#[cortex_m_rt::pre_init]
unsafe fn before_main() {
    _ = hprintln!("! CM4: before_main(): Hardware initialization complete...");
}

/// CM4 "main" function
/// 
/// Demonstates how to use "safe rust" to access peripherals by taking ownership
/// of the `cyt2b7::Peripherals` instance.
/// The demo periodically toggles LED4 (port 19, pin 0)
#[cortex_m_rt::entry]
fn main() -> ! {
    use crate::get_core_frequency;

    _ = hprintln!("! CM4: Entering main()...");

    // Core peripheral registers
    let cp = cortex_m::Peripherals::take().unwrap();
    let syst = cp.SYST;

    let gpio = config_gpio();

    let mut delay = Delay::new(syst, get_core_frequency());

    loop {
        // Invert GPIO state once every 250ms
        gpio.prt19.out_inv.write(|w| w.out0().bit(true));
        delay.delay_ms(250);
    }
}

/// Set-up the relevant GPIO port/pin for the LED
fn config_gpio() -> pac::GPIO {
    // Peripheral registers
    let p = pac::Peripherals::take().unwrap();
    let gpio = p.GPIO;

    let strong_value: u8 = GPIO::prt::cfg::DRIVE_MODE0_A::STRONG.into();
    gpio.prt19.cfg.write(|w| w.drive_mode0().bits(strong_value));

    gpio
}
