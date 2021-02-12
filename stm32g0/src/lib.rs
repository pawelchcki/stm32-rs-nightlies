//! Peripheral access API for STM32G0 microcontrollers
//! (generated using [svd2rust](https://github.com/rust-embedded/svd2rust)
//! 0.17.0)
//!
//! You can find an overview of the API here:
//! [svd2rust/#peripheral-api](https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api)
//!
//! For more details see the README here:
//! [stm32-rs](https://github.com/stm32-rs/stm32-rs)
//!
//! This crate supports all STM32G0 devices; for the complete list please
//! see:
//! [stm32g0](https://github.com/stm32-rs/stm32-rs/tree/master/stm32g0)
//!
//! Due to doc build limitations, not all devices may be shown on docs.rs;
//! a representative few have been selected instead. For a complete list of
//! available registers and fields see: [stm32-rs Device Coverage](https://stm32.agg.io/rs)

#![allow(non_camel_case_types)]
#![no_std]

mod generic;
pub use self::generic::*;

#[cfg(feature = "stm32g030")]
pub mod stm32g030;

#[cfg(feature = "stm32g031")]
pub mod stm32g031;

#[cfg(feature = "stm32g041")]
pub mod stm32g041;

#[cfg(feature = "stm32g070")]
pub mod stm32g070;

#[cfg(feature = "stm32g071")]
pub mod stm32g071;

#[cfg(feature = "stm32g081")]
pub mod stm32g081;

