//! utils to support implementing bit string traits

pub use self::bigendian::*;

mod bigendian;

#[cfg(test)]
mod bigendian_tests;
