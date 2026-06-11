#![cfg_attr(not(test), no_std)]
#![allow(dead_code)]

use core::ops::Deref;

pub mod bus;
mod constants;
mod endpoint;
mod endpoint_memory;
mod endpoint_registers;
pub mod pac;
mod registers;

pub enum UsbSpeed {
    FullSpeed,
    HighSpeed,
}

/// A trait for device-specific USB peripherals. Implement this to add support for a new hardware
/// platform. Peripherals that have this trait must have the same register block as LPC USBFS
/// peripherals.
pub trait UsbPeripheral: Deref<Target = crate::pac::usb::RegisterBlock> + Send + Sync {
    /// Pointer to the register block
    const REGISTERS: *const ();

    const SPEED: UsbSpeed;
}
