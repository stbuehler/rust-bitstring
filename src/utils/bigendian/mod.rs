//! bigendian utils to support implementing bit string traits on unsigned integers

#[cfg(feature = "bigendian")]
pub use self::int_helpers::{
	u128,
	u16,
	u32,
	u8,
};

#[cfg(any(feature = "bigendian", feature = "net"))]
pub(crate) mod int_helpers;

#[cfg(feature = "bigendian")]
mod trait_impls;
#[cfg(feature = "bigendian")]
pub(crate) mod traits;

#[cfg(any(feature = "bigendian", feature = "net"))]
#[cfg(test)]
mod tests;
