/// Generic helper methods to treat unsigned integer `u*`-slices as big endian bit strings.
pub trait BigEndianBitString: Sized {
	/// bits in a single element
	const ELEMENT_BITS: usize;

	/// integer with single bit set. bit 0 is the highest bit (big
	/// endian).  Wraps at `Self::ELEMENT_BITS()`.
	fn mask(ndx: usize) -> Self;

	/// increment from right; don't touch first `prefix` bits; returns
	/// true on overflow
	///
	/// # Panics
	///
	/// Panics if `prefix > Self::ELEMENT_BITS() * slice.len()`.
	fn inc(slice: &mut [Self], prefix: usize) -> bool;

	/// Get the `ndx`th bit.
	///
	/// # Panics
	///
	/// Panics if `ndx >= Self::ELEMENT_BITS() * slice.len()`.
	fn get(slice: &[Self], ndx: usize) -> bool;

	/// Set the `ndx`th bit to `bit`.
	///
	/// # Panics
	///
	/// Panics if `ndx >= Self::ELEMENT_BITS() * slice.len()`.
	fn set(slice: &mut [Self], ndx: usize, bit: bool);

	/// Flips the `ndx`th bit.
	///
	/// # Panics
	///
	/// Panics if `ndx >= Self::ELEMENT_BITS() * slice.len()`.
	fn flip(slice: &mut [Self], ndx: usize);

	/// Length of the longest shared prefix of two bit strings.
	fn shared_prefix_len(slice: &[Self], other: &[Self], max_len: usize) -> usize;

	/// Set all bits from [ndx..] to `false`.
	///
	/// Doesn't do anything if `ndx >= Self::ELEMENT_BITS() * slice.len()`.
	fn set_false_from(slice: &mut [Self], ndx: usize);

	/// Whether all bits from [ndx..] are `false`.
	///
	/// Returns `true` if `ndx >= Self::ELEMENT_BITS() * slice.len()`.
	fn is_false_from(slice: &[Self], ndx: usize) -> bool;

	/// Set all bits from [ndx..] to `true`.
	///
	/// Doesn't do anything if `ndx >= Self::ELEMENT_BITS() * slice.len()`.
	fn set_true_from(slice: &mut [Self], ndx: usize);

	/// Whether all bits from [ndx..] are `true`.
	///
	/// Returns `true` if `ndx >= Self::ELEMENT_BITS() * slice.len()`.
	fn is_true_from(slice: &[Self], ndx: usize) -> bool;

	/// check whether another bit string `other` shares the first
	/// `prefix` bits with `self`
	fn contains(slice: &[Self], prefix: usize, other: &[Self]) -> bool;
}

macro_rules! impl_big_endian_for {
	($mod:ident => $t:ty) => {
		/// `BigEndianBitString` functions for specific integer type
		pub mod $mod {
			use std::{
				cmp::min,
				mem::size_of,
			};

			/// bits in a single element
			pub const ELEMENT_BITS: usize = 8 * size_of::<$t>();

			/// integer with single bit set. bit 0 is the highest bit (big
			/// endian).  Wraps at `ELEMENT_BITS`.
			pub const fn mask(ndx: usize) -> $t {
				let bit_ndx = ELEMENT_BITS - 1 - (ndx % ELEMENT_BITS);
				1 << bit_ndx
			}

			/// increment from right; don't touch first `prefix` bits; returns
			/// true on overflow
			///
			/// # Panics
			///
			/// Panics if `prefix > ELEMENT_BITS * slice.len()`.
			pub fn inc(slice: &mut [$t], prefix: usize) -> bool {
				assert!(prefix <= ELEMENT_BITS * slice.len());
				// first element that might be touched by overflow
				let slice_ndx = prefix / ELEMENT_BITS;
				if 0 == prefix % ELEMENT_BITS {
					// just don't overflow into elems before slice_ndx

					let mut overflow = true;
					for i in (slice_ndx..slice.len()).rev() {
						let (r, o) = slice[i].overflowing_add(1);
						slice[i] = r;
						overflow = o;
						if !overflow {
							break;
						}
					}
					overflow
				} else {
					// on overflow set all bits to false from "prefix"th bit

					let last_prefix_bit_mask = mask(prefix - 1);
					let overflow_mask = last_prefix_bit_mask - 1;
					let overflow_elem = slice[slice_ndx] & !overflow_mask;

					let mut overflow = true;
					for i in (slice_ndx..slice.len()).rev() {
						let (r, o) = slice[i].overflowing_add(1);
						slice[i] = r;
						overflow = o;
						if !overflow {
							break;
						}
					}

					// touched last bit in prefix? -> overflow
					if overflow_elem & last_prefix_bit_mask
						!= slice[slice_ndx] & last_prefix_bit_mask
					{
						// restore bits at slice_ndx which belong to
						// prefix. as an overflow just happened, the
						// remaining bits must be 0.
						slice[slice_ndx] = overflow_elem;

						true
					} else {
						assert!(!overflow, "can't overflow without touching prefix");
						false
					}
				}
			}

			/// Get the `ndx`th bit.
			///
			/// # Panics
			///
			/// Panics if `ndx >= ELEMENT_BITS * slice.len()`.
			pub const fn get(slice: &[$t], ndx: usize) -> bool {
				let mask = mask(ndx);
				let slice_ndx = ndx / ELEMENT_BITS;
				0 != (slice[slice_ndx] & mask)
			}

			/// Set the `ndx`th bit to `bit`.
			///
			/// # Panics
			///
			/// Panics if `ndx >= ELEMENT_BITS * slice.len()`.
			pub fn set(slice: &mut [$t], ndx: usize, bit: bool) {
				let mask = mask(ndx);
				let slice_ndx = ndx / ELEMENT_BITS;
				if bit {
					slice[slice_ndx] |= mask;
				} else {
					slice[slice_ndx] &= !mask;
				}
			}

			/// Flips the `ndx`th bit.
			///
			/// # Panics
			///
			/// Panics if `ndx >= ELEMENT_BITS * slice.len()`.
			pub fn flip(slice: &mut [$t], ndx: usize) {
				let mask = mask(ndx);
				let slice_ndx = ndx / ELEMENT_BITS;
				slice[slice_ndx] ^= mask;
			}

			/// Length of the longest shared prefix of two bit strings.
			pub fn shared_prefix_len(slice: &[$t], other: &[$t], max_len: usize) -> usize {
				if 0 == max_len {
					return 0;
				}
				// slice index of last bit to compare
				let slice_ndx = (max_len - 1) / ELEMENT_BITS;
				for i in 0..slice_ndx {
					let diff = slice[i] ^ other[i];
					if 0 != diff {
						return i * ELEMENT_BITS + diff.leading_zeros() as usize;
					}
				}
				let diff = slice[slice_ndx] ^ other[slice_ndx];
				if 0 != diff {
					min(
						max_len,
						slice_ndx * ELEMENT_BITS + diff.leading_zeros() as usize,
					)
				} else {
					max_len
				}
			}

			/// Set all bits from [ndx..] to `false`.
			///
			/// Doesn't do anything if `ndx >= ELEMENT_BITS * slice.len()`.
			pub fn set_false_from(slice: &mut [$t], ndx: usize) {
				let slice_ndx = ndx / ELEMENT_BITS;
				if 0 == ndx % ELEMENT_BITS {
					for i in slice_ndx..slice.len() {
						slice[i] = 0;
					}
				} else if slice_ndx < slice.len() {
					let mask = mask(ndx - 1) - 1;
					slice[slice_ndx] &= !mask;
					for i in slice_ndx + 1..slice.len() {
						slice[i] = 0;
					}
				}
			}

			/// Whether all bits from [ndx..] are `false`.
			///
			/// Returns `true` if `ndx >= ELEMENT_BITS * slice.len()`.
			pub fn is_false_from(slice: &[$t], ndx: usize) -> bool {
				let slice_ndx = ndx / ELEMENT_BITS;
				if 0 == ndx % ELEMENT_BITS {
					for i in slice_ndx..slice.len() {
						if 0 != slice[i] {
							return false;
						}
					}
				} else if slice_ndx < slice.len() {
					let mask = mask(ndx - 1) - 1;
					if 0 != slice[slice_ndx] & mask {
						return false;
					}
					for i in slice_ndx + 1..slice.len() {
						if 0 != slice[i] {
							return false;
						}
					}
				}
				true
			}

			/// Set all bits from [ndx..] to `true`.
			///
			/// Doesn't do anything if `ndx >= ELEMENT_BITS * slice.len()`.
			pub fn set_true_from(slice: &mut [$t], ndx: usize) {
				let slice_ndx = ndx / ELEMENT_BITS;
				if 0 == ndx % ELEMENT_BITS {
					for i in slice_ndx..slice.len() {
						slice[i] = !0;
					}
				} else if slice_ndx < slice.len() {
					let mask = mask(ndx - 1) - 1;
					slice[slice_ndx] |= mask;
					for i in slice_ndx + 1..slice.len() {
						slice[i] = !0;
					}
				}
			}

			/// Whether all bits from [ndx..] are `true`.
			///
			/// Returns `true` if `ndx >= ELEMENT_BITS * slice.len()`.
			pub fn is_true_from(slice: &[$t], ndx: usize) -> bool {
				let slice_ndx = ndx / ELEMENT_BITS;
				if 0 == ndx % ELEMENT_BITS {
					for i in slice_ndx..slice.len() {
						if slice[i] != !0 {
							return false;
						}
					}
				} else if slice_ndx < slice.len() {
					let mask = mask(ndx - 1) - 1;
					if slice[slice_ndx] | !mask != !0 {
						return false;
					}
					for i in slice_ndx + 1..slice.len() {
						if slice[i] != !0 {
							return false;
						}
					}
				}
				true
			}

			/// check whether another bit string `other` shares the first
			/// `prefix` bits with `self`
			pub fn contains(slice: &[$t], prefix: usize, other: &[$t]) -> bool {
				let slice_ndx = prefix / ELEMENT_BITS;
				for i in 0..slice_ndx {
					if slice[i] != other[i] {
						return false;
					}
				}
				if 0 == prefix % ELEMENT_BITS {
					return true;
				}
				let mask = !(mask(prefix - 1) - 1);
				0 == mask & (slice[slice_ndx] ^ other[slice_ndx])
			}
		}

		impl BigEndianBitString for $t {
			const ELEMENT_BITS: usize = $mod::ELEMENT_BITS;

			fn mask(ndx: usize) -> Self {
				$mod::mask(ndx)
			}

			fn inc(slice: &mut [Self], prefix: usize) -> bool {
				$mod::inc(slice, prefix)
			}

			fn get(slice: &[Self], ndx: usize) -> bool {
				$mod::get(slice, ndx)
			}

			fn set(slice: &mut [Self], ndx: usize, bit: bool) {
				$mod::set(slice, ndx, bit)
			}

			fn flip(slice: &mut [Self], ndx: usize) {
				$mod::flip(slice, ndx)
			}

			fn shared_prefix_len(slice: &[Self], other: &[Self], max_len: usize) -> usize {
				$mod::shared_prefix_len(slice, other, max_len)
			}

			fn set_false_from(slice: &mut [Self], ndx: usize) {
				$mod::set_false_from(slice, ndx)
			}

			fn is_false_from(slice: &[Self], ndx: usize) -> bool {
				$mod::is_false_from(slice, ndx)
			}

			fn set_true_from(slice: &mut [Self], ndx: usize) {
				$mod::set_true_from(slice, ndx)
			}

			fn is_true_from(slice: &[Self], ndx: usize) -> bool {
				$mod::is_true_from(slice, ndx)
			}

			fn contains(slice: &[Self], prefix: usize, other: &[Self]) -> bool {
				$mod::contains(slice, prefix, other)
			}
		}
	};
}

impl_big_endian_for! {u8 => u8}
impl_big_endian_for! {u16 => u16}
impl_big_endian_for! {u32 => u32}
impl_big_endian_for! {u64 => u64}
impl_big_endian_for! {u128 => u128}

#[cfg(test)]
mod tests {
	use super::BigEndianBitString;

	#[test]
	fn shared_prefix() {
		assert_eq!(0, u8::shared_prefix_len(&[0b0000_0000], &[0b0000_0000], 0));
		assert_eq!(0, u8::shared_prefix_len(&[0b0000_0000], &[0b1000_0000], 8));
		assert_eq!(1, u8::shared_prefix_len(&[0b0000_0000], &[0b0000_0000], 1));
		assert_eq!(1, u8::shared_prefix_len(&[0b0000_0000], &[0b0100_0000], 8));
		assert_eq!(7, u8::shared_prefix_len(&[0b1100_0000], &[0b1100_0001], 7));
		assert_eq!(7, u8::shared_prefix_len(&[0b1100_0000], &[0b1100_0001], 8));

		assert_eq!(
			0,
			u8::shared_prefix_len(&[0b0000_0000, 0b0000_0000], &[0b0000_0000, 0b0000_0000], 0)
		);
		assert_eq!(
			0,
			u8::shared_prefix_len(&[0b0000_0000, 0b0000_0000], &[0b1000_0000, 0b0000_0000], 8)
		);
		assert_eq!(
			1,
			u8::shared_prefix_len(&[0b0000_0000, 0b0000_0000], &[0b0000_0000, 0b0000_0000], 1)
		);
		assert_eq!(
			1,
			u8::shared_prefix_len(&[0b0000_0000, 0b0000_0000], &[0b0100_0000, 0b0000_0000], 8)
		);
		assert_eq!(
			7,
			u8::shared_prefix_len(&[0b1100_0000, 0b0000_0000], &[0b1100_0001, 0b0000_0000], 7)
		);
		assert_eq!(
			7,
			u8::shared_prefix_len(&[0b1100_0000, 0b0000_0000], &[0b1100_0001, 0b0000_0000], 8)
		);

		assert_eq!(
			8,
			u8::shared_prefix_len(&[0b0010_1000, 0b0000_0000], &[0b0010_1000, 0b0000_0000], 8)
		);
		assert_eq!(
			8,
			u8::shared_prefix_len(&[0b0010_1000, 0b0000_0000], &[0b0010_1000, 0b1000_0000], 16)
		);
		assert_eq!(
			9,
			u8::shared_prefix_len(&[0b0010_1000, 0b0000_0000], &[0b0010_1000, 0b0000_0000], 9)
		);
		assert_eq!(
			9,
			u8::shared_prefix_len(&[0b0010_1000, 0b0000_0000], &[0b0010_1000, 0b0100_0000], 16)
		);
		assert_eq!(
			15,
			u8::shared_prefix_len(&[0b0010_1000, 0b1100_0000], &[0b0010_1000, 0b1100_0001], 15)
		);
		assert_eq!(
			15,
			u8::shared_prefix_len(&[0b0010_1000, 0b1100_0000], &[0b0010_1000, 0b1100_0001], 16)
		);
	}

	fn u8_inc<S: AsMut<[u8]>>(mut slice: S, prefix: usize) -> (bool, S) {
		let overflow = u8::inc(slice.as_mut(), prefix);
		(overflow, slice)
	}
	#[test]
	fn inc() {
		// make sure overflow doesn't change the fixed prefix
		assert_eq!(
			(true, [0b0000_0000, 0b0000_0000]),
			u8_inc([0b0000_0000, 0b0000_0000], 16)
		);
		assert_eq!(
			(false, [0b0000_0000, 0b0000_0001]),
			u8_inc([0b0000_0000, 0b0000_0000], 15)
		);
		assert_eq!(
			(true, [0b0000_0000, 0b0000_0000]),
			u8_inc([0b0000_0000, 0b0000_0001], 15)
		);
		assert_eq!(
			(true, [0b0000_0000, 0b0000_1010]),
			u8_inc([0b0000_0000, 0b0000_1011], 15)
		);
		assert_eq!(
			(true, [0b0000_0000, 0b0000_1110]),
			u8_inc([0b0000_0000, 0b0000_1111], 15)
		);
		assert_eq!(
			(true, [0b0000_0000, 0b1111_1110]),
			u8_inc([0b0000_0000, 0b1111_1111], 15)
		);
		assert_eq!(
			(true, [0b0000_0001, 0b1111_1110]),
			u8_inc([0b0000_0001, 0b1111_1111], 15)
		);
		assert_eq!(
			(false, [0b0000_0000, 0b0000_0001]),
			u8_inc([0b0000_0000, 0b0000_0000], 8)
		);
		assert_eq!(
			(true, [0b0000_0000, 0b0000_0000]),
			u8_inc([0b0000_0000, 0b1111_1111], 8)
		);
		assert_eq!(
			(true, [0b0000_0001, 0b0000_0000]),
			u8_inc([0b0000_0001, 0b1111_1111], 8)
		);
		assert_eq!(
			(false, [0b0000_0000, 0b0000_0001]),
			u8_inc([0b0000_0000, 0b0000_0000], 0)
		);
		assert_eq!(
			(true, [0b0000_0000, 0b0000_0000]),
			u8_inc([0b1111_1111, 0b1111_1111], 0)
		);
	}
}
