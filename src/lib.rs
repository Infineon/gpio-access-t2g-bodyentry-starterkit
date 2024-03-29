// \file lib.rs
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

#![no_std]
#![no_main]

use panic_halt as _;

use cyt2b7 as pac;
use pac::CPUSS;
use pac::SRSS;
use pac::FLASHC;
use pac::PERI;
use pac::SCB0;

#[cfg(all(cm4))]
pub mod cortex_m4;

#[cfg(all(cm0))]
pub mod cortex_m0;

#[cfg(all(cm4))]
const CORE_FREQUENCY: u32 = 160_000_000;

#[cfg(all(cm0))]
const CORE_FREQUENCY: u32 = 80_000_000;

unsafe fn setup_memory_wait_states() {
    // ROM wait states...
    (*CPUSS::ptr()).rom_ctl.write(|w| w.slow_ws().bits(1));
    (*CPUSS::ptr()).rom_ctl.write(|w| w.fast_ws().bits(0));
    
    // RAM 0 wait states...
    (*CPUSS::ptr()).ram0_ctl0.write(|w| w.slow_ws().bits(1));
    (*CPUSS::ptr()).ram0_ctl0.write(|w| w.fast_ws().bits(0));

    // RAM 1 wait states...
    (*CPUSS::ptr()).ram1_ctl0.write(|w| w.slow_ws().bits(1));
    (*CPUSS::ptr()).ram1_ctl0.write(|w| w.fast_ws().bits(0));

    // Flash wait states...
    (*FLASHC::ptr()).flash_ctl.write(|w| w.main_ws().bits(1));
}

/// Unlock the watchdog timer (WDT)
pub unsafe fn unlock_wdt() {
    (*SRSS::ptr()).wdt.lock.write(|w| w.wdt_lock().bits(1));
    (*SRSS::ptr()).wdt.lock.write(|w| w.wdt_lock().bits(2));
}

/// Lock the watchdog timer (WDT)
pub unsafe fn lock_wdt() {
    (*SRSS::ptr()).wdt.lock.write(|w| w.wdt_lock().bits(3));
}

/// Return core freqeuncy for the current core
pub fn get_core_frequency() -> u32 {
    CORE_FREQUENCY
}

/// Initialize all clocks using Internal Main Oscillator (IMO) as the System clock 
pub unsafe fn config_sys_clk() {
    // Disable the watchdog...
    unlock_wdt();
    (*SRSS::ptr()).wdt.ctl.write(|w| w.enable().bit(false));
    lock_wdt();

    setup_memory_wait_states();

    // Set LF clock source...
    (*SRSS::ptr()).clk_select.write(|w| w.lfclk_sel().bits(0));

    // Set CPUSS dividers...
    // - FAST (CM4) = 160,000,000
    // - PERI (CM0) = Divided by 2
    // - SLOW (CM0) == PERI (CM0)
    (*CPUSS::ptr()).cm4_clock_ctl.write(|w| w.fast_int_div().bits(0));
    (*CPUSS::ptr()).cm0_clock_ctl.write(|w| w.bits(0x01000000));

    // Set and enable PLL0...
    // - FEEDBACK_DIV = 1
    // - REFERENCE_DIV = 40
    // - OUTPUT_DIV = 2
    // - ENABLE = 1
    (*SRSS::ptr()).clk_path_select[1].write(|w| w.path_mux().bits(0));
    (*SRSS::ptr()).clk_pll_config[0].write(|w| w.bits(0x80020128));

    // Wait for a PLL lock...
    while (*SRSS::ptr()).clk_pll_status[0].read().locked().bit_is_clear() {}

    // Set path 2 source
    (*SRSS::ptr()).clk_path_select[2].write(|w| w.path_mux().bits(0));

    // Enable HF0 clock with PLL0 as source and 'ROOT_DIV = NO_DIV'...
    (*SRSS::ptr()).clk_root_select[0].write(|w| w.bits(0x80000001));

    // Enable HF1 clock with PLL0 as source and 'ROOT_DIV = DIV_BY_2'...
    (*SRSS::ptr()).clk_root_select[1].write(|w| w.bits(0x80000011));

    // Enable ILO0...
    unlock_wdt();
    (*SRSS::ptr()).clk_ilo0_config.write(|w| w.enable().bit(true));
    (*SRSS::ptr()).clk_ilo0_config.write(|w| w.ilo0_backup().bit(true));
    lock_wdt();
}

/// Initialize SCB0 clock and dividers for 115200 baudrate
pub unsafe fn config_scb_clk() {
    // Select divider '0' and specify 24.5 fractional divider type
    (*PERI::PTR).clock_ctl[20].modify(|_, w| w
        .div_sel().bits(0)
        .type_sel().bits(3)
    );

    // Set integer and fractional parts of the 24.5 divider
    // The following formula is used:
    //      CLK_PERI / ((INT24_DIV + 1) + (FRAC5_DIV / 32)) / OVS) = BPS
    //
    // so, with 'OVS = 8' and 'CLK_PERI = 80MHz':
    //      80MHz / 86.8125 / 8 = ~115191
    (*PERI::PTR).div_24_5_ctl[0].write(|w| w
        .int24_div().bits(85)
        .frac5_div().bits(26)
    );

    // For the divider number selected above ('DIV_SEL = 0' and 'TYPE_SEL = 3'),
    // set CLK_PERI as the reference clock for alignment and then enable the
    // divider
    (*PERI::PTR).div_cmd.modify(|_, w| w
        .div_sel().bits(0)
        .type_sel().bits(3)
        .pa_type_sel().bits(3)
        .pa_div_sel().bits(0xFF)
        .enable().bit(true)
    );
}

/// Reset SCB0 registers to default values
pub unsafe fn deinit_scb_uart() {
    (*SCB0::PTR).ctrl.write(|w| w.bits(0x0300000F));
    (*SCB0::PTR).uart_ctrl.write(|w| w.bits(0x00300000));

    (*SCB0::PTR).uart_rx_ctrl.write(|w| w.bits(0));
    (*SCB0::PTR).rx_ctrl.write(|w| w.bits(0x00000107));
    (*SCB0::PTR).rx_fifo_ctrl.write(|w| w.bits(0));
    (*SCB0::PTR).rx_match.write(|w| w.bits(0));

    (*SCB0::PTR).uart_tx_ctrl.write(|w| w.bits(0));
    (*SCB0::PTR).tx_ctrl.write(|w| w.bits(0x00000107));
    (*SCB0::PTR).tx_fifo_ctrl.write(|w| w.bits(0));

    (*SCB0::PTR).uart_flow_ctrl.write(|w| w.bits(0));

    (*SCB0::PTR).intr_spi_ec_mask.write(|w| w.bits(0));
    (*SCB0::PTR).intr_i2c_ec_mask.write(|w| w.bits(0));
    (*SCB0::PTR).intr_rx_mask.write(|w| w.bits(0));
    (*SCB0::PTR).intr_tx_mask.write(|w| w.bits(0));
    (*SCB0::PTR).intr_m_mask.write(|w| w.bits(0));
    (*SCB0::PTR).intr_s_mask.write(|w| w.bits(0));
}

/// Setup SCB0 for UART functionality
pub unsafe fn init_scb_uart() {
    // SCB0 CTRL register:
    // - address matching disabled
    // - memory width 8 bits
    // - oversampling 8 - 1
    // - mode UART
    (*SCB0::PTR).ctrl.modify(|_, w| w
        .addr_accept().bit(false)
        .mem_width().bits(0)
        .ovs().bits(7)
        .mode().bits(2)
    );

    // SCB0 UART_CTRL, mode set to standard UART
    (*SCB0::PTR).uart_ctrl.modify(|_, w| w
        .mode().bits(0)
    );

    // SCB0 UART_RX_CTRL register:
    // - polarity 0 (normal/non-invented)
    // - multiprocessor mode off
    // - drop on parity/frame error disabled
    // - break width (0x0f) + 1 == 0
    // - parity 0, parity disabled
    (*SCB0::PTR).uart_rx_ctrl.modify(|_, w| w
        .polarity().bit(false)
        .mp_mode().bit(false)
        .drop_on_parity_error().bit(false)
        .drop_on_frame_error().bit(false)
        .break_width().bits(0x0f)
        .stop_bits().bits(1)
        .parity().bit(false)
        .parity_enabled().bit(false)
    );

    // SCB0 RX_CTRL register:
    // - LSB first
    // - median filter disabled
    // - data width + 1 = 8
    (*SCB0::PTR).rx_ctrl.modify(|_, w| w
        .msb_first().bit(false)
        .median().bit(false)
        .data_width().bits(7)
    );

    // SCB0 RX_MATCH register:
    // - address and mask 0 
    (*SCB0::PTR).rx_match.modify(|_, w| w
        .addr().bits(0)
        .mask().bits(0)
    );

    // SCB0 UART_TX_CTRL register:
    // - retry-on-nack disabled
    // - stop bits 1
    // - parity 0, parity disabled
    (*SCB0::PTR).uart_tx_ctrl.modify(|_, w| w
        .retry_on_nack().bit(false)
        .stop_bits().bits(1)
        .parity().bit(false)
        .parity_enabled().bit(false)
    );

    // SCB0 TX_CTRL register:
    // - LSB first
    // - data width + 1 = 8
    // - open drain disabled
    (*SCB0::PTR).tx_ctrl.modify(|_, w| w
        .msb_first().bit(false)
        .data_width().bits(7)
        .open_drain().bit(false)
    );

    // SCB0 RX_FIFO_CTRL register:
    // - trigger level 0
    (*SCB0::PTR).rx_fifo_ctrl.modify(|_, w| w
        .trigger_level().bits(0)
    );

    // SCB0 UART_FLOW_CTRL register:
    // - CTS disabled
    // - CTS polarity active low
    // - RTS polarity active low
    // - trigger level 0 (RTS effectively disabled)
    (*SCB0::PTR).uart_flow_ctrl.modify(|_, w| w
        .cts_enabled().bit(false)
        .cts_polarity().bit(false)
        .rts_polarity().bit(false)
        .trigger_level().bits(0)
    );

    // SCB0 TX_FIFO_CTRL register:
    // - trigger level 0
    (*SCB0::PTR).tx_fifo_ctrl.modify(|_, w| w
        .trigger_level().bits(0)
    );
}

/// Enable SCB0 block
pub unsafe fn enable_scb() {
    (*SCB0::PTR).ctrl.modify(|_, w| w
        .enabled().bit(true)
    );
}

/// Returns true if SCB0 UART TX FIO is empty
pub unsafe fn is_uart_tx_fifo_empty() -> bool {
    (*SCB0::PTR).tx_fifo_status.read().sr_valid().bit_is_clear() &&
        (*SCB0::PTR).tx_fifo_status.read().used().bits().eq(&0)
}

/// Write to SCB0 UART TX FIFO
pub unsafe fn uart_tx_fifo_write(byte: u8) {
    (*SCB0::PTR).tx_fifo_wr.write(|w| w.bits(byte as u32));
}

/// Get SCB0 UART RX count
pub unsafe fn uart_rx_fifo_count()  -> u16 {
    (*SCB0::PTR).rx_fifo_status.read().used().bits()
}

/// Write to SCB0 UART TX FIFO
pub unsafe fn uart_rx_fifo_read() -> u8 {
    (*SCB0::PTR).rx_fifo_rd.read().bits() as u8
}

/// Initialize the vector table for the CM4 core in the CPUSS_CM4_VECTOR_TABLE_BASE
/// register with the start address of the vector table and then enable power to
/// the CM4 core
pub unsafe fn enable_cm4() {
    // Set the CM4 vector table to the start of address of the vector table,
    // which is at the beginning of the FLASH assigned to the CM4 core (see
    // the memory_cm4.x linker file). This has to be done before starting the
    // CM4 core
    (*CPUSS::ptr())
        .cm4_vector_table_base
        .write(|w| w.bits(0x10008000));

    // Start the CM4 core
    (*CPUSS::ptr()).cm4_pwr_ctl.write(|w| w.bits(0x05fa0003) );
}
