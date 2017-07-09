//! This crate provides traits to treat various objects as bit strings.
#![warn(missing_docs)]
#![doc(html_root_url = "https://docs.rs/bitstring/0.1.0")]

pub use self::bit_length_string::*;

// re-export. sadly duplicates the doc pages...
pub use self::bit_string::BitString;
pub use self::fixed_bit_string::FixedBitString;

mod address;
mod bit_length_string;
pub mod bit_string;
pub mod fixed_bit_string;
pub mod utils;
