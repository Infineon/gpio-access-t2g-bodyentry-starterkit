#![no_main]
#![no_std]
#![allow(unused_imports)]

use cyt2b7;
use panic_halt as _;

#[cfg(all(cm0))]
use traveo_rust_demo::cortex_m0 as traveo_core;

#[cfg(all(cm4))]
use traveo_rust_demo::cortex_m4 as traveo_core;



