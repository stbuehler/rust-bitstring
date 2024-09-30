//! This crate provides traits to treat various objects as bit strings.
//!
//! ## Features
//!
//! - `fixed`: enables [`FixedBitString`] and [`fixed_bit_string`]
//! - `bigendian`: enables [`utils::BigEndianBitString`] and [`utils::bigendian`]
//! - `net`: implements [`FixedBitString`] for [`Ipv4Addr`] and [`Ipv6Addr`]
//!
//! [`Ipv4Addr`]: core::net::Ipv4Addr
//! [`Ipv6Addr`]: core::net::Ipv6Addr
#![warn(missing_docs)]
#![doc(html_root_url = "https://docs.rs/bitstring/0.2.0")]
#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub use self::bit_string::BitString;

#[cfg(feature = "fixed")]
pub use self::{
	bit_length_string::BitLengthString,
	fixed_bit_string::traits::FixedBitString,
};

mod bit_string;

#[cfg(feature = "net")]
mod address;

pub mod utils;

#[cfg(feature = "fixed")]
mod bit_length_string;
#[cfg(feature = "fixed")]
pub mod fixed_bit_string;
