#![no_std]
#![doc = include_str!("../README.md")]

pub mod linux;
pub mod win;

pub use aragonite_macros::aragonite_main;
