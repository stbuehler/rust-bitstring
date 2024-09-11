//! This crate provides traits to treat various objects as bit strings.
#![warn(missing_docs)]
#![doc(html_root_url = "https://docs.rs/bitstring/0.2.0-alpha1")]
#![no_std]

pub use self::bit_length_string::*;

// re-export. sadly duplicates the doc pages...
pub use self::{
	bit_string::BitString,
	fixed_bit_string::traits::FixedBitString,
};

mod address;
mod bit_length_string;
pub mod bit_string;
pub mod fixed_bit_string;
pub mod utils;
