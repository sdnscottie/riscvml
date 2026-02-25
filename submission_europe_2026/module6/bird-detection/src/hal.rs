//! Rust HAL Layer — Type-Safe Hardware Abstractions
//!
//! This module provides the Rust HAL (Hardware Abstraction Layer) used
//! throughout the bird detection pipeline. It wraps esp-hal and esp-idf-hal
//! types to provide:
//!
//!   - Compile-time peripheral ownership (prevents data races on hardware)
//!   - Type-safe GPIO, SPI, I2C, UART pin configuration
//!   - DMA channel abstractions with borrow-checker enforcement
//!   - Peripheral singleton pattern ensuring exclusive access
//!
//! The HAL layer sits between the application logic (main.rs) and the
//! FFI boundary (ffi.rs), providing safe Rust interfaces for all
//! hardware interactions.

use esp_hal::gpio::{Input, Output, PushPull, Floating};
use esp_hal::i2c::I2c;
use esp_hal::spi::master::Spi;

/// Peripheral set — owned singletons for all hardware used by the pipeline.
///
/// Rust's ownership model guarantees that each peripheral is held by
/// exactly one component at a time. Attempting to use a peripheral
/// that's already been moved causes a compile-time error.
pub struct Peripherals {
    pub i2c0: I2c<'static>,
    pub camera_dma: DmaChannel,
    pub display_dma: DmaChannel,
    pub encoder_dma: DmaChannel,
}

/// DMA channel abstraction with compile-time ownership tracking.
///
/// A DMA channel can be in one of two states:
///   - `Idle`: available for configuration
///   - `Active`: transfer in progress, buffer is borrowed
///
/// The borrow checker prevents reconfiguring an active channel or
/// accessing the buffer while DMA is reading it.
pub struct DmaChannel {
    _channel_id: u8,
}

impl DmaChannel {
    /// Start a DMA transfer. The buffer is borrowed for the duration.
    ///
    /// Returns a `DmaTransfer` handle that must be awaited or dropped
    /// before the buffer can be reused — enforced at compile time.
    pub fn start_transfer<'buf>(
        &mut self,
        buffer: &'buf [u8],
    ) -> DmaTransfer<'buf> {
        // TODO: Configure DMA descriptors
        // TODO: Start transfer via HAL
        todo!("DMA transfer start")
    }
}

/// Active DMA transfer — holds a borrow on the source buffer.
pub struct DmaTransfer<'buf> {
    _buffer: &'buf [u8],
}

impl<'buf> DmaTransfer<'buf> {
    /// Await transfer completion via Embassy async interrupt.
    pub async fn wait(self) {
        // TODO: Await DMA done interrupt
        todo!("DMA wait")
    }
}

/// Type-safe GPIO pin configuration.
///
/// Pins are configured at init time and their types encode their mode:
/// `Output<PushPull>` vs `Input<Floating>` — calling `.set_high()` on
/// an input pin is a compile-time error.
pub struct GpioPins {
    pub led_r: Output<'static, PushPull>,
    pub led_g: Output<'static, PushPull>,
    pub led_b: Output<'static, PushPull>,
    pub servo_sda: Output<'static, PushPull>,
    pub servo_scl: Output<'static, PushPull>,
}
