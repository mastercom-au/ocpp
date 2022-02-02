//! # ocpp
//!
//! This library is intended to provide a framework for serialising and deserialising OCPP packets as rust types.
//! The original OCPP protocol is an [Open Charge Alliance ](https://www.openchargealliance.org/) project

pub mod common;
pub use common::common_types;
pub mod point_init;
pub mod server_init;

#[cfg(test)]
pub mod test;
#[macro_use]
extern crate lazy_static;
