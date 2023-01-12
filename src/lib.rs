#![no_std]
#![no_main]

use panic_halt as _;

#[cfg(all(cm4))]
pub mod cortex_m4;

#[cfg(all(cm0))]
pub mod cortex_m0;

