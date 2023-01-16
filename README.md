<img src="./media/ifx-logo-600.gif" align="right" width="150" />  

#  gpio-access-t2g-entry-starterkit
**A simple, baremetal example, written in Rust, which toggles one LED periodically and the other based on the status of a switch**

## Device
The device used in this example is Traveo™ CYT2B75.

## Board
The board used for testing is the Traveo™ II Entry Family Starter Kit.

## Scope of work
This tutorial demonstrates the usage of Traveo™ T2G Peripheral Access Crates _(TODO: link to correct PAL crate)_ to access GPIO registers from both cores:
- From the startup core, CM0, LED1 is turned ON by default and is turned OFF whenever SW1 is pressed. All peripherals access is done using using _unsafe Rust_ (i.e. inside `unsafe` blocks)
- From the application core, CM4, LED4 is toggled periodically. All peripheral access is done using _safe Rust_

## Introduction
The Traveo™ CYT2B75 contains 2 ARM Cortex-M cores:  
1. The  _**startup core**_ is a Cortex-M0+, referred to as CM0 in the rest of this tutorial for brevity. It is responsible for initial HW setup and for enabling the _application core_
2. The _**application core**_ is a more performant Cortex-M4, referred to CM4 in the rest of this tutorial

The CM0 demo code...
- initializes it's own vector table
- initializes the vector table for the CM4 core
- starts the CM4 core
- sets up GPIO 12.2 as output for LED1 and GPIO 7.0 as input for SW1
- turns on LED1 and monitors SW1, turning off LED1 whenever SW1 is pressed

The CM4 demo code...
- initializes GPIO 19.0 as output for LED2
- sets up a delay source
- toggles LED2 periodically

This example demonstrates peripheral access using both safe and unsafe Rust. All peripheral access in CM0 is done using _unsafe Rust_, e.g. raw pointer dereferencing with no memory safety checks. All such accesses are enclosed in `unsafe` blocks. On the other hand, peripheral access from CM4 is done using _safe Rust_.

For the sake of simplicity, no mutual exclusion for peripheral access is implemented in this tutorial.

The Peripheral Access Crates used to access peripheral register are generated from Traveo™ T2G SVD file using the `svd2rust`. For more information about the `svd2rust` API, please refer to it's documentation and for details of Traveo™ peripheral registers, please refer to the appropriate Technical Reference Manual - links are provided in the __References__ section at the end.

## Hardware setup  
This code example has been developed for the board CYTVII-B-E-1M-SK (Traveo™ II Entry Family Starter Kit).

<img src="./media/traveo-ii-entryfamily-starterkit.jpg" width="800" />

The development board is powered-on and debugged using a micro-USB cable connected to the PC. It comes with the __KitProg3__ debug interface, so no external debugger is needed.

## Develoment environment
A list of all requirements is given below. For detailed steps please refer to the setup instruction (TODO: link to the setup-instructions repository).

- VSCode with the following extensions:
    - Cortex-Debug
    - rust-analyzer
- Rust toolchains:
    - stable-x86_64-pc-windows-gnu
- Rust Embedded Targets:
    - thumbv6m-none-eabi (Cortex-M0+)
    - thumbv7em-none-eabi (Cortex-M4)
- Rust tools:
    - cargo-binutils
    - cargo-generate
- Arm GNU Toolchain for Windows (arm-none-eabi-gdb)
- Python 2.7
- CypressAutoFlashUtility + OpenOCD
    - Included in the repository under the `traveo_debug` folder

## Implementation

## Run and Test

## References
