use std::{
	cmp::min,
	mem::size_of,
};

/// Generic helper methods to treat [u*]-slices as big endian bit
/// strings.
pub trait BigEndianBitString: Sized {
	/// bits in a single element
	fn elembits() -> usize {
		8 * size_of::<Self>()
	}

	/// integer with single bit set. bit 0 is the highest bit (big
	/// endian).  Wraps at `Self::elembits()`.
	fn mask(ndx: usize) -> Self;

	/// increment from right; don't touch first `prefix` bits; returns
	/// true on overflow
	///
	/// # Panics
	///
	/// Panics if `prefix > Self::elembits() * slice.len()`.
	fn inc(slice: &mut [Self], prefix: usize) -> bool;

	/// Get the `ndx`th bit.
	///
	/// # Panics
	///
	/// Panics if `ndx >= Self::elembits() * slice.len()`.
	fn get(slice: &[Self], ndx: usize) -> bool;

	/// Set the `ndx`th bit to `bit`.
	///
	/// # Panics
	///
	/// Panics if `ndx >= Self::elembits() * slice.len()`.
	fn set(slice: &mut [Self], ndx: usize, bit: bool) {
		if bit {
			Self::on(slice, ndx)
		} else {
			Self::off(slice, ndx)
		}
	}

	/// Set the `ndx`th bit to `true`.
	///
	/// # Panics
	///
	/// Panics if `ndx >= Self::elembits() * slice.len()`.
	fn on(slice: &mut [Self], ndx: usize);

	/// Set the `ndx`th bit to `false`.
	///
	/// # Panics
	///
	/// Panics if `ndx >= Self::elembits() * slice.len()`.
	fn off(slice: &mut [Self], ndx: usize);

	/// Flips the `ndx`th bit.
	///
	/// # Panics
	///
	/// Panics if `ndx >= Self::elembits() * slice.len()`.
	fn flip(slice: &mut [Self], ndx: usize);

	/// Length of the longest shared prefix of two bit strings.
	fn shared_prefix_len(slice: &[Self], other: &[Self], max_len: usize) -> usize;

	/// Set all bits from [ndx..] to `false`.
	///
	/// Doesn't do anything if `ndx >= Self::elembits() * slice.len()`.
	fn set_false_from(slice: &mut [Self], ndx: usize);

	/// Whether all bits from [ndx..] are `false`.
	///
	/// Returns `true` if `ndx >= Self::elembits() * slice.len()`.
	fn is_false_from(slice: &[Self], ndx: usize) -> bool;

	/// Set all bits from [ndx..] to `true`.
	///
	/// Doesn't do anything if `ndx >= Self::elembits() * slice.len()`.
	fn set_true_from(slice: &mut [Self], ndx: usize);

	/// Whether all bits from [ndx..] are `true`.
	///
	/// Returns `true` if `ndx >= Self::elembits() * slice.len()`.
	fn is_true_from(slice: &[Self], ndx: usize) -> bool;

	/// check whether another bit string `other` shares the first
	/// `prefix` bits with `self`
	fn contains(slice: &[Self], prefix: usize, other: &[Self]) -> bool;
}

macro_rules! impl_big_endian_for {
	($t:ty) => {
		impl BigEndianBitString for $t {
			fn mask(ndx: usize) -> Self {
				let bits = Self::elembits();
				let bit_ndx = bits - 1 - (ndx % bits);
				1 << bit_ndx
			}

			fn inc(slice: &mut [Self], prefix: usize) -> bool {
				assert!(prefix <= Self::elembits() * slice.len());
				// first element that might be touched by overflow
				let slice_ndx = prefix / Self::elembits();
				if 0 == prefix % Self::elembits() {
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

					let last_prefix_bit_mask = Self::mask(prefix - 1);
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

			fn get(slice: &[Self], ndx: usize) -> bool {
				let mask = Self::mask(ndx);
				let slice_ndx = ndx / Self::elembits();
				0 != (slice[slice_ndx] & mask)
			}

			fn on(slice: &mut [Self], ndx: usize) {
				let mask = Self::mask(ndx);
				let slice_ndx = ndx / Self::elembits();
				slice[slice_ndx] |= mask;
			}

			fn off(slice: &mut [Self], ndx: usize) {
				let mask = Self::mask(ndx);
				let slice_ndx = ndx / Self::elembits();
				slice[slice_ndx] &= !mask;
			}

			fn flip(slice: &mut [Self], ndx: usize) {
				let mask = Self::mask(ndx);
				let slice_ndx = ndx / Self::elembits();
				slice[slice_ndx] ^= mask;
			}

			fn shared_prefix_len(slice: &[Self], other: &[Self], max_len: usize) -> usize {
				if 0 == max_len {
					return 0;
				}
				// slice index of last bit to compare
				let slice_ndx = (max_len - 1) / Self::elembits();
				for i in 0..slice_ndx {
					let diff = slice[i] ^ other[i];
					if 0 != diff {
						return i * Self::elembits() + diff.leading_zeros() as usize;
					}
				}
				let diff = slice[slice_ndx] ^ other[slice_ndx];
				if 0 != diff {
					min(
						max_len,
						slice_ndx * Self::elembits() + diff.leading_zeros() as usize,
					)
				} else {
					max_len
				}
			}

			fn set_false_from(slice: &mut [Self], ndx: usize) {
				let slice_ndx = ndx / Self::elembits();
				if 0 == ndx % Self::elembits() {
					for i in slice_ndx..slice.len() {
						slice[i] = 0;
					}
				} else if slice_ndx < slice.len() {
					let mask = Self::mask(ndx - 1) - 1;
					slice[slice_ndx] &= !mask;
					for i in slice_ndx + 1..slice.len() {
						slice[i] = 0;
					}
				}
			}

			fn is_false_from(slice: &[Self], ndx: usize) -> bool {
				let slice_ndx = ndx / Self::elembits();
				if 0 == ndx % Self::elembits() {
					for i in slice_ndx..slice.len() {
						if 0 != slice[i] {
							return false;
						}
					}
				} else if slice_ndx < slice.len() {
					let mask = Self::mask(ndx - 1) - 1;
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

			fn set_true_from(slice: &mut [Self], ndx: usize) {
				let slice_ndx = ndx / Self::elembits();
				if 0 == ndx % Self::elembits() {
					for i in slice_ndx..slice.len() {
						slice[i] = !0;
					}
				} else if slice_ndx < slice.len() {
					let mask = Self::mask(ndx - 1) - 1;
					slice[slice_ndx] |= mask;
					for i in slice_ndx + 1..slice.len() {
						slice[i] = !0;
					}
				}
			}

			fn is_true_from(slice: &[Self], ndx: usize) -> bool {
				let slice_ndx = ndx / Self::elembits();
				if 0 == ndx % Self::elembits() {
					for i in slice_ndx..slice.len() {
						if slice[i] != !0 {
							return false;
						}
					}
				} else if slice_ndx < slice.len() {
					let mask = Self::mask(ndx - 1) - 1;
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

			fn contains(slice: &[Self], prefix: usize, other: &[Self]) -> bool {
				let slice_ndx = prefix / Self::elembits();
				for i in 0..slice_ndx {
					if slice[i] != other[i] {
						return false;
					}
				}
				if 0 == prefix % Self::elembits() {
					return true;
				}
				let mask = !(Self::mask(prefix - 1) - 1);
				0 == mask & (slice[slice_ndx] ^ other[slice_ndx])
			}
		}
	};
}

impl_big_endian_for! {u8}
impl_big_endian_for! {u16}
impl_big_endian_for! {u32}
impl_big_endian_for! {u64}

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
