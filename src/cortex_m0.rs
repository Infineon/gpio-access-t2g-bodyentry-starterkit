// \file cortex_m0.rs
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

use panic_halt as _;
use cortex_m_rt::__RESET_VECTOR;
use cortex_m_semihosting::hprintln;

use cyt2b7 as pac;
use pac::gpio as GPIO;
use pac::SCB;

/// Executes before the main function and can be used for HW initialization
///
/// 1. Sets up the interrupt vector table Cortex-M0+ (CM0) core
/// 2. Initializes clocks
/// 3. Enables the Cortex-M4 (CM4) core
#[cortex_m_rt::pre_init]
unsafe fn before_main() {
    use crate::config_sys_clk;
    use crate::enable_cm4;

    // Initialize the CM0 vector table address in the CM0P_SCS_VTOR register
    let vtor_addr = &(__RESET_VECTOR) as *const unsafe extern "C" fn() -> !;
    (*SCB::PTR).vtor.write(vtor_addr as u32 - 4);

    // Initialize clocks
    config_sys_clk();

    // Enable the CM4 core
    enable_cm4();

    _ = hprintln!("! CM0: before_main(): Hardware initialization complete...");
}

/// CM0 "main" function
/// 
/// Demonstrates `unsafe` access to perihpheral registers.
/// The demo turns on LED1 (port 12, pin 2) by default and turns it off as
/// long as SW1 (port 7, pin 0) is kept pressed.
#[cortex_m_rt::entry]
fn main() -> ! {
    _ = hprintln!("! CM0: Entering main()...");

    unsafe {
        let gpio = &*pac::GPIO::PTR;
                
        config_gpio(gpio);

        loop {
            if (*gpio).prt7.in_.read().in0().bit_is_clear() {
                (*gpio).prt12.out_clr.write(|w| w.out2().set_bit());
            }
            else {
                (*gpio).prt12.out_set.write(|w| w.out2().set_bit());
            }
        }
	}
}

/// Set-up the relevant GPIO port/pins for LED4 and SW1
fn config_gpio(gpio: *const GPIO::RegisterBlock) {
    let high_z_value: u8 = GPIO::prt::cfg::DRIVE_MODE0_A::HIGHZ.into();
    let strong_value: u8 = GPIO::prt::cfg::DRIVE_MODE0_A::STRONG.into();

    unsafe {    
        (*gpio).prt12.cfg.modify(|_, w| w.drive_mode2().bits(strong_value));

        (*gpio).prt7.cfg.modify(|_, w| w 
            .drive_mode0().bits(high_z_value)
            .in_en0().bit(true)
        );
    }
}
