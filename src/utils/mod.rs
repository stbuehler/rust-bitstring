//! utils to support implementing bit string traits

#[cfg(feature = "bigendian")]
pub use bigendian::traits::BigEndianBitString;

#[cfg(feature = "bigendian")]
pub mod bigendian;

#[cfg(not(feature = "bigendian"))]
pub(crate) mod bigendian;
