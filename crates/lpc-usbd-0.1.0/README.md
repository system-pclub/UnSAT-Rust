[![crates.io](https://img.shields.io/crates/d/lpc-usbd.svg)](https://crates.io/crates/lpc-usbd)
[![crates.io](https://img.shields.io/crates/v/lpc-usbd.svg)](https://crates.io/crates/lpc-usbd)
![Build Status](https://github.com/lpc-rs/lpc-usbd/workflows/CI/badge.svg)

# `lpc-usbd`

> [usb-device](https://github.com/mvirkkunen/usb-device) implementation for LPC
microcontrollers.

This project is inspired from the great work over at [lpc55-hal](https://github.com/lpc55/lpc55-hal).

## Supported microcontrollers

* `LPC546xx`
* `LPC55xx`
* And others?

## Usage

This driver is intended for use through a device hal library.
Such hal library should implement `UsbPeripheral` for the corresponding USB peripheral object.
This trait declares all the peripheral properties that may vary from one device family to the other.

## Examples

See the `hal` example for the reference hal implementation.
