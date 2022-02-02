//! # ocpp
//! 
//! This library is intended to provide a framework for serialising and deserialising OCPP packets as rust types.
//! The original OCPP protocol is an [Open Charge Alliance project](https://www.openchargealliance.org/)
//! 
//! # point_init
//! [point_init] collection of messages initiatied by the charge point
//! 
//! # server_init
//! [server_init] is collection of messages initiatied by the central server
//! 
//! # common_types
//! [common_types] contains a variety of common types found in multiple structures. Aggregated to reduce code duplication.

pub mod common_types;
pub mod point_init;
pub mod server_init;

#[cfg(test)]
pub mod test;
#[macro_use]
extern crate lazy_static;
