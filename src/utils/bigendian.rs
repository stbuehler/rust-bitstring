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

	/// Set all bits from [ndx..] to `false` (`0`).
	///
	/// Doesn't do anything if `ndx >= Self::ELEMENT_BITS() * slice.len()`.
	fn set_false_from(slice: &mut [Self], ndx: usize);

	/// Whether all bits from [ndx..] are `false` (`0`).
	///
	/// Returns `true` if `ndx >= Self::ELEMENT_BITS() * slice.len()`.
	fn is_false_from(slice: &[Self], ndx: usize) -> bool;

	/// Set all bits from [ndx..] to `true` (`1`).
	///
	/// Doesn't do anything if `ndx >= Self::ELEMENT_BITS() * slice.len()`.
	fn set_true_from(slice: &mut [Self], ndx: usize);

	/// Whether all bits from [ndx..] are `true` (`1`).
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
			#[inline]
			pub const fn mask(ndx: usize) -> $t {
				let bit_ndx = ELEMENT_BITS - 1 - (ndx % ELEMENT_BITS);
				1 << bit_ndx
			}

			const fn mask_suffix(ndx: usize) -> $t {
				assert!(ndx <= ELEMENT_BITS);
				if ndx >= ELEMENT_BITS {
					0
				} else {
					!0 >> ndx
				}
			}

			/// increment from right; don't touch first `prefix` bits; returns
			/// true on overflow
			///
			/// # Panics
			///
			/// Panics if `prefix > ELEMENT_BITS`.
			pub const fn element_inc(value: $t, prefix: usize) -> ($t, bool) {
				assert!(prefix <= ELEMENT_BITS);
				if prefix == ELEMENT_BITS {
					return (value, true);
				}
				if prefix == 0 {
					return value.overflowing_add(1);
				}

				let result = value.wrapping_add(1);

				let fixed_bits_mask = !mask_suffix(prefix);

				if (result ^ value) & fixed_bits_mask != 0 {
					// overflow: set all non-fixed bits to false (from "prefix"th bit)
					return (value & fixed_bits_mask, true);
				}
				(result, false)
			}

			/// increment from right; don't touch first `prefix` bits; returns
			/// true on overflow
			///
			/// # Panics
			///
			/// Panics if `prefix > ELEMENT_BITS * slice.len()`.
			pub fn slice_inc(slice: &mut [$t], prefix: usize) -> bool {
				let slice_ndx = prefix / ELEMENT_BITS;
				let element_ndx = prefix % ELEMENT_BITS;
				if slice_ndx >= slice.len() {
					assert!(element_ndx == 0);
					return true;
				}

				for i in (slice_ndx + 1..slice.len()).rev() {
					let overflow;
					(slice[i], overflow) = slice[i].overflowing_add(1);
					if !overflow {
						return false;
					}
				}

				let overflow;
				(slice[slice_ndx], overflow) = element_inc(slice[slice_ndx], element_ndx);
				overflow
			}

			/// Get the `ndx`th bit.
			///
			/// # Panics
			///
			/// Panics if `ndx >= ELEMENT_BITS`.
			pub const fn element_get(value: $t, ndx: usize) -> bool {
				assert!(ndx < ELEMENT_BITS);
				0 != value & mask(ndx)
			}

			/// Get the `ndx`th bit.
			///
			/// # Panics
			///
			/// Panics if `ndx >= ELEMENT_BITS * slice.len()`.
			pub const fn slice_get(slice: &[$t], ndx: usize) -> bool {
				let slice_ndx = ndx / ELEMENT_BITS;
				let element_ndx = ndx % ELEMENT_BITS;
				element_get(slice[slice_ndx], element_ndx)
			}

			/// Set the `ndx`th bit to `bit`.
			///
			/// # Panics
			///
			/// Panics if `ndx >= ELEMENT_BITS`.
			pub const fn element_set(value: $t, ndx: usize, bit: bool) -> $t {
				assert!(ndx < ELEMENT_BITS);
				let mask = mask(ndx);
				if bit {
					value | mask
				} else {
					value & !mask
				}
			}

			/// Set the `ndx`th bit to `bit`.
			///
			/// # Panics
			///
			/// Panics if `ndx >= ELEMENT_BITS * slice.len()`.
			pub fn slice_set(slice: &mut [$t], ndx: usize, bit: bool) {
				let slice_ndx = ndx / ELEMENT_BITS;
				slice[slice_ndx] = element_set(slice[slice_ndx], ndx % ELEMENT_BITS, bit);
			}

			/// Flips the `ndx`th bit.
			///
			/// # Panics
			///
			/// Panics if `ndx >= ELEMENT_BITS`.
			pub const fn element_flip(value: $t, ndx: usize) -> $t {
				assert!(ndx < ELEMENT_BITS);
				value ^ mask(ndx)
			}

			/// Flips the `ndx`th bit.
			///
			/// # Panics
			///
			/// Panics if `ndx >= ELEMENT_BITS * slice.len()`.
			pub fn slice_flip(slice: &mut [$t], ndx: usize) {
				let slice_ndx = ndx / ELEMENT_BITS;
				slice[slice_ndx] = element_flip(slice[slice_ndx], ndx % ELEMENT_BITS);
			}

			/// Length of the longest shared prefix of two bit strings.
			pub fn element_shared_prefix_len(value: $t, other: $t, max_len: usize) -> usize {
				assert!(max_len <= ELEMENT_BITS);
				min((value ^ other).leading_zeros() as usize, max_len)
			}

			/// Length of the longest shared prefix of two bit strings.
			pub fn slice_shared_prefix_len(slice: &[$t], other: &[$t], max_len: usize) -> usize {
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

			/// Set all bits from [ndx..] to `false` (`0`).
			///
			/// Doesn't do anything if `ndx >= ELEMENT_BITS`.
			pub fn element_set_false_from(value: $t, ndx: usize) -> $t {
				if ndx >= ELEMENT_BITS {
					return value;
				}
				value & !mask_suffix(ndx)
			}

			/// Set all bits from [ndx..] to `false` (`0`).
			///
			/// Doesn't do anything if `ndx >= ELEMENT_BITS * slice.len()`.
			pub fn slice_set_false_from(slice: &mut [$t], ndx: usize) {
				let slice_ndx = ndx / ELEMENT_BITS;
				if slice_ndx >= slice.len() {
					return;
				}
				slice[slice_ndx] = element_set_false_from(slice[slice_ndx], ndx % ELEMENT_BITS);
				for i in slice_ndx + 1..slice.len() {
					slice[i] = 0;
				}
			}

			/// Whether all bits from [ndx..] are `false` (`0`).
			///
			/// Returns `true` if `ndx >= ELEMENT_BITS`.
			pub const fn element_is_false_from(value: $t, ndx: usize) -> bool {
				if ndx >= ELEMENT_BITS {
					return true;
				}
				0 == value & mask_suffix(ndx)
			}

			/// Whether all bits from [ndx..] are `false` (`0`).
			///
			/// Returns `true` if `ndx >= ELEMENT_BITS * slice.len()`.
			pub fn slice_is_false_from(slice: &[$t], ndx: usize) -> bool {
				let slice_ndx = ndx / ELEMENT_BITS;
				if slice_ndx >= slice.len() {
					return true;
				}
				if !element_is_false_from(slice[slice_ndx], ndx % ELEMENT_BITS) {
					return false;
				}
				slice[slice_ndx + 1..].iter().all(|&value| value == 0)
			}

			/// Set all bits from [ndx..] to `true` (`1`).
			///
			/// Doesn't do anything if `ndx >= ELEMENT_BITS`.
			pub fn element_set_true_from(value: $t, ndx: usize) -> $t {
				if ndx >= ELEMENT_BITS {
					return value;
				}
				value | mask_suffix(ndx)
			}

			/// Set all bits from [ndx..] to `true` (`1`).
			///
			/// Doesn't do anything if `ndx >= ELEMENT_BITS * slice.len()`.
			pub fn slice_set_true_from(slice: &mut [$t], ndx: usize) {
				let slice_ndx = ndx / ELEMENT_BITS;
				if slice_ndx >= slice.len() {
					return;
				}
				slice[slice_ndx] = element_set_true_from(slice[slice_ndx], ndx % ELEMENT_BITS);
				for i in slice_ndx + 1..slice.len() {
					slice[i] = !0;
				}
			}

			/// Whether all bits from [ndx..] are `true` (`1`).
			///
			/// Returns `true` if `ndx >= ELEMENT_BITS`.
			pub const fn element_is_true_from(value: $t, ndx: usize) -> bool {
				if ndx >= ELEMENT_BITS {
					return true;
				}
				!0 == value | !mask_suffix(ndx)
			}

			/// Whether all bits from [ndx..] are `true` (`1`).
			///
			/// Returns `true` if `ndx >= ELEMENT_BITS * slice.len()`.
			pub fn slice_is_true_from(slice: &[$t], ndx: usize) -> bool {
				let slice_ndx = ndx / ELEMENT_BITS;
				if slice_ndx >= slice.len() {
					return true;
				}
				if !element_is_true_from(slice[slice_ndx], ndx % ELEMENT_BITS) {
					return false;
				}
				slice[slice_ndx + 1..].iter().all(|&value| value == !0)
			}

			/// check whether another bit string `other` shares the first
			/// `prefix` bits with `value`
			///
			/// # Panics
			///
			/// Panics if `prefix >= ELEMENT_BITS`.
			pub const fn element_contains(value: $t, prefix: usize, other: $t) -> bool {
				let mask = !mask_suffix(prefix);
				0 == mask & (value ^ other)
			}

			/// check whether another bit string `other` shares the first
			/// `prefix` bits with `slice`
			///
			/// # Panics
			///
			/// Panics if `prefix >= ELEMENT_BITS * slice.len()`.
			pub fn slice_contains(slice: &[$t], prefix: usize, other: &[$t]) -> bool {
				let slice_ndx = prefix / ELEMENT_BITS;
				for i in 0..slice_ndx {
					if slice[i] != other[i] {
						return false;
					}
				}
				let element_ndx = prefix % ELEMENT_BITS;
				if 0 == element_ndx {
					return true;
				}
				element_contains(slice[slice_ndx], element_ndx, other[slice_ndx])
			}
		}

		impl BigEndianBitString for $t {
			const ELEMENT_BITS: usize = $mod::ELEMENT_BITS;

			fn mask(ndx: usize) -> Self {
				$mod::mask(ndx)
			}

			fn inc(slice: &mut [Self], prefix: usize) -> bool {
				$mod::slice_inc(slice, prefix)
			}

			fn get(slice: &[Self], ndx: usize) -> bool {
				$mod::slice_get(slice, ndx)
			}

			fn set(slice: &mut [Self], ndx: usize, bit: bool) {
				$mod::slice_set(slice, ndx, bit)
			}

			fn flip(slice: &mut [Self], ndx: usize) {
				$mod::slice_flip(slice, ndx)
			}

			fn shared_prefix_len(slice: &[Self], other: &[Self], max_len: usize) -> usize {
				$mod::slice_shared_prefix_len(slice, other, max_len)
			}

			fn set_false_from(slice: &mut [Self], ndx: usize) {
				$mod::slice_set_false_from(slice, ndx)
			}

			fn is_false_from(slice: &[Self], ndx: usize) -> bool {
				$mod::slice_is_false_from(slice, ndx)
			}

			fn set_true_from(slice: &mut [Self], ndx: usize) {
				$mod::slice_set_true_from(slice, ndx)
			}

			fn is_true_from(slice: &[Self], ndx: usize) -> bool {
				$mod::slice_is_true_from(slice, ndx)
			}

			fn contains(slice: &[Self], prefix: usize, other: &[Self]) -> bool {
				$mod::slice_contains(slice, prefix, other)
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

	fn u8_slice_inc<S: AsMut<[u8]>>(mut slice: S, prefix: usize) -> (bool, S) {
		let overflow = super::u8::slice_inc(slice.as_mut(), prefix);
		(overflow, slice)
	}

	fn u8_slice_set<S: AsMut<[u8]>>(mut slice: S, ndx: usize, bit: bool) -> S {
		super::u8::slice_set(slice.as_mut(), ndx, bit);
		slice
	}

	fn u8_slice_flip<S: AsMut<[u8]>>(mut slice: S, ndx: usize) -> S {
		super::u8::slice_flip(slice.as_mut(), ndx);
		slice
	}

	fn u8_slice_set_false_from<S: AsMut<[u8]>>(mut slice: S, ndx: usize) -> S {
		super::u8::slice_set_false_from(slice.as_mut(), ndx);
		slice
	}

	fn u8_slice_set_true_from<S: AsMut<[u8]>>(mut slice: S, ndx: usize) -> S {
		super::u8::slice_set_true_from(slice.as_mut(), ndx);
		slice
	}

	#[test]
	fn test_u8_element_inc() {
		assert_eq!(super::u8::element_inc(0b0000_0000, 0), (0b0000_0001, false));
		assert_eq!(super::u8::element_inc(0b0000_0000, 4), (0b0000_0001, false));
		assert_eq!(super::u8::element_inc(0b0000_0000, 8), (0b0000_0000, true));
		assert_eq!(super::u8::element_inc(0b0000_1000, 0), (0b0000_1001, false));
		assert_eq!(super::u8::element_inc(0b0000_1000, 4), (0b0000_1001, false));
		assert_eq!(super::u8::element_inc(0b0000_1000, 8), (0b0000_1000, true));
		assert_eq!(super::u8::element_inc(0b0000_1111, 0), (0b0001_0000, false));
		assert_eq!(super::u8::element_inc(0b0000_1111, 4), (0b0000_0000, true));
		assert_eq!(super::u8::element_inc(0b0000_1111, 8), (0b0000_1111, true));
		assert_eq!(super::u8::element_inc(0b0001_1111, 0), (0b0010_0000, false));
		assert_eq!(super::u8::element_inc(0b0001_1111, 4), (0b0001_0000, true));
		assert_eq!(super::u8::element_inc(0b0001_1111, 8), (0b0001_1111, true));
		assert_eq!(super::u8::element_inc(0b1111_1111, 0), (0b0000_0000, true));
		assert_eq!(super::u8::element_inc(0b1111_1111, 4), (0b1111_0000, true));
		assert_eq!(super::u8::element_inc(0b1111_1111, 8), (0b1111_1111, true));
	}

	#[test]
	fn test_u8_element_set() {
		assert_eq!(super::u8::element_set(0b0000_0000, 0, true), 0b1000_0000);
		assert_eq!(super::u8::element_set(0b1000_0000, 0, true), 0b1000_0000);
		assert_eq!(super::u8::element_set(0b0000_0000, 4, true), 0b0000_1000);
		assert_eq!(super::u8::element_set(0b0000_1000, 4, true), 0b0000_1000);
		assert_eq!(super::u8::element_set(0b0000_0000, 7, true), 0b0000_0001);
		assert_eq!(super::u8::element_set(0b0000_0001, 7, true), 0b0000_0001);

		assert_eq!(super::u8::element_set(0b1000_0000, 0, false), 0b0000_0000);
		assert_eq!(super::u8::element_set(0b0000_0000, 0, false), 0b0000_0000);
		assert_eq!(super::u8::element_set(0b0000_1000, 4, false), 0b0000_0000);
		assert_eq!(super::u8::element_set(0b0000_0000, 4, false), 0b0000_0000);
		assert_eq!(super::u8::element_set(0b0000_0001, 7, false), 0b0000_0000);
		assert_eq!(super::u8::element_set(0b0000_0000, 7, false), 0b0000_0000);
	}

	#[test]
	fn test_u8_element_flip() {
		assert_eq!(super::u8::element_flip(0b0000_0000, 0), 0b1000_0000);
		assert_eq!(super::u8::element_flip(0b1000_0000, 0), 0b0000_0000);
		assert_eq!(super::u8::element_flip(0b0000_0000, 4), 0b0000_1000);
		assert_eq!(super::u8::element_flip(0b0000_1000, 4), 0b0000_0000);
		assert_eq!(super::u8::element_flip(0b0000_0000, 7), 0b0000_0001);
		assert_eq!(super::u8::element_flip(0b0000_0001, 7), 0b0000_0000);
	}

	#[test]
	fn test_u8_element_shared_prefix_len() {
		assert_eq!(super::u8::element_shared_prefix_len(0b0000_0000, 0b0000_0000, 0), 0);
		assert_eq!(super::u8::element_shared_prefix_len(0b0000_0000, 0b1000_0000, 8), 0);
		assert_eq!(super::u8::element_shared_prefix_len(0b0000_0000, 0b0000_0000, 1), 1);
		assert_eq!(super::u8::element_shared_prefix_len(0b0000_0000, 0b0100_0000, 8), 1);
		assert_eq!(super::u8::element_shared_prefix_len(0b1100_0000, 0b1100_0001, 7), 7);
		assert_eq!(super::u8::element_shared_prefix_len(0b1100_0000, 0b1100_0001, 8), 7);
		assert_eq!(super::u8::element_shared_prefix_len(0b0000_0000, 0b0000_0000, 8), 8);
		assert_eq!(super::u8::element_shared_prefix_len(0b1100_0001, 0b1100_0001, 8), 8);
		assert_eq!(super::u8::element_shared_prefix_len(0b1111_1111, 0b1111_1111, 8), 8);
	}

	#[test]
	fn test_u8_element_get() {
		assert_eq!(super::u8::element_get(0b0000_0000, 0), false);
		assert_eq!(super::u8::element_get(0b1000_0000, 0), true);
		assert_eq!(super::u8::element_get(0b0000_0000, 1), false);
		assert_eq!(super::u8::element_get(0b0100_0000, 1), true);
		assert_eq!(super::u8::element_get(0b0000_0000, 2), false);
		assert_eq!(super::u8::element_get(0b0010_0000, 2), true);
		assert_eq!(super::u8::element_get(0b0000_0000, 3), false);
		assert_eq!(super::u8::element_get(0b0001_0000, 3), true);
		assert_eq!(super::u8::element_get(0b0000_0000, 6), false);
		assert_eq!(super::u8::element_get(0b0000_0010, 6), true);
		assert_eq!(super::u8::element_get(0b0000_0000, 7), false);
		assert_eq!(super::u8::element_get(0b0000_0001, 7), true);
	}

	#[test]
	fn test_u8_element_is_false_from() {
		assert!(super::u8::element_is_false_from(0b0000_0000, 0));
		assert!(!super::u8::element_is_false_from(0b1111_1111, 0));
		assert!(super::u8::element_is_false_from(0b0000_0000, 1));
		assert!(super::u8::element_is_false_from(0b1000_0000, 1));
		assert!(!super::u8::element_is_false_from(0b1111_1111, 1));
		assert!(super::u8::element_is_false_from(0b0001_0000, 4));
		assert!(super::u8::element_is_false_from(0b1111_0000, 4));
		assert!(!super::u8::element_is_false_from(0b0000_1111, 4));
		assert!(super::u8::element_is_false_from(0b1111_1110, 7));
		assert!(!super::u8::element_is_false_from(0b1111_1111, 7));
		assert!(super::u8::element_is_false_from(0b1111_1111, 8));
	}

	#[test]
	fn test_u8_element_set_false_from() {
		assert_eq!(
			super::u8::element_set_false_from(0b0000_0000, 0),
			0b0000_0000
		);
		assert_eq!(
			super::u8::element_set_false_from(0b1111_1111, 0),
			0b0000_0000
		);
		assert_eq!(
			super::u8::element_set_false_from(0b1111_0000, 4),
			0b1111_0000
		);
		assert_eq!(
			super::u8::element_set_false_from(0b1111_1111, 4),
			0b1111_0000
		);
		assert_eq!(
			super::u8::element_set_false_from(0b0000_1111, 8),
			0b0000_1111
		);
		assert_eq!(
			super::u8::element_set_false_from(0b1111_0000, 8),
			0b1111_0000
		);
		assert_eq!(
			super::u8::element_set_false_from(0b1111_1111, 8),
			0b1111_1111
		);
	}

	#[test]
	fn test_u8_element_is_true_from() {
		assert!(super::u8::element_is_true_from(0b1111_1111, 0));
		assert!(!super::u8::element_is_true_from(0b0000_0000, 0));
		assert!(super::u8::element_is_true_from(0b0111_1111, 1));
		assert!(super::u8::element_is_true_from(0b1111_1111, 1));
		assert!(!super::u8::element_is_true_from(0b1000_0000, 1));
		assert!(super::u8::element_is_true_from(0b1110_1111, 4));
		assert!(super::u8::element_is_true_from(0b0000_1111, 4));
		assert!(!super::u8::element_is_true_from(0b1111_0000, 4));
		assert!(super::u8::element_is_true_from(0b0000_0001, 7));
		assert!(!super::u8::element_is_true_from(0b0000_0000, 7));
		assert!(super::u8::element_is_true_from(0b0000_0000, 8));
	}

	#[test]
	fn test_u8_element_set_true_from() {
		assert_eq!(
			super::u8::element_set_true_from(0b0000_0000, 0),
			0b1111_1111
		);
		assert_eq!(
			super::u8::element_set_true_from(0b1111_1111, 0),
			0b1111_1111
		);
		assert_eq!(
			super::u8::element_set_true_from(0b0000_0000, 4),
			0b0000_1111
		);
		assert_eq!(
			super::u8::element_set_true_from(0b0000_1111, 4),
			0b0000_1111
		);
		assert_eq!(
			super::u8::element_set_true_from(0b0000_1111, 8),
			0b0000_1111
		);
		assert_eq!(
			super::u8::element_set_true_from(0b1111_0000, 8),
			0b1111_0000
		);
		assert_eq!(
			super::u8::element_set_true_from(0b0000_0000, 8),
			0b0000_0000
		);
	}

	#[test]
	fn test_u8_element_contains() {
		assert!(super::u8::element_contains(0b0000_0000, 0, 0b0000_0000));
		assert!(super::u8::element_contains(0b1000_0000, 0, 0b0000_0000));
		assert!(super::u8::element_contains(0b0000_0000, 0, 0b1000_0000));
		assert!(super::u8::element_contains(0b0000_1000, 0, 0b0000_0000));
		assert!(super::u8::element_contains(0b0000_0000, 0, 0b1000_1000));
		assert!(super::u8::element_contains(0b0000_0001, 0, 0b0000_0000));
		assert!(super::u8::element_contains(0b0000_0000, 0, 0b1000_0001));
		assert!(super::u8::element_contains(0b0000_0000, 1, 0b0111_1111));
		assert!(super::u8::element_contains(0b1111_1111, 1, 0b1000_0000));
		assert!(super::u8::element_contains(0b0000_0000, 1, 0b0111_1111));
		assert!(super::u8::element_contains(0b1111_1111, 1, 0b1000_0000));
		assert!(super::u8::element_contains(0b0000_0000, 7, 0b0000_0001));
		assert!(super::u8::element_contains(0b1111_1111, 7, 0b1111_1110));
		assert!(!super::u8::element_contains(0b0000_0000, 7, 0b1000_0001));
		assert!(!super::u8::element_contains(0b0000_0000, 7, 0b0000_1001));
		assert!(!super::u8::element_contains(0b0111_1111, 7, 0b1111_1110));
		assert!(!super::u8::element_contains(0b1111_1111, 7, 0b1111_0110));
		assert!(super::u8::element_contains(0b0000_0000, 8, 0b0000_0000));
		assert!(super::u8::element_contains(0b1111_1111, 8, 0b1111_1111));
		assert!(!super::u8::element_contains(0b0000_0000, 8, 0b0000_0001));
		assert!(!super::u8::element_contains(0b0000_0000, 8, 0b1000_0000));
		assert!(!super::u8::element_contains(0b1111_1111, 8, 0b1111_1110));
		assert!(!super::u8::element_contains(0b1111_1111, 8, 0b0111_1111));
	}

	#[test]
	fn test_u8_slice_inc() {
		// make sure overflow doesn't change the fixed prefix
		assert_eq!(
			u8_slice_inc([0b0000_0000, 0b0000_0000], 16),
			(true, [0b0000_0000, 0b0000_0000])
		);
		assert_eq!(
			u8_slice_inc([0b0000_0000, 0b0000_0000], 15),
			(false, [0b0000_0000, 0b0000_0001]),
		);
		assert_eq!(
			u8_slice_inc([0b0000_0000, 0b0000_0001], 15),
			(true, [0b0000_0000, 0b0000_0000]),
		);
		assert_eq!(
			u8_slice_inc([0b0000_0000, 0b0000_1011], 15),
			(true, [0b0000_0000, 0b0000_1010]),
		);
		assert_eq!(
			u8_slice_inc([0b0000_0000, 0b0000_1111], 15),
			(true, [0b0000_0000, 0b0000_1110]),
		);
		assert_eq!(
			u8_slice_inc([0b0000_0000, 0b1111_1111], 15),
			(true, [0b0000_0000, 0b1111_1110]),
		);
		assert_eq!(
			u8_slice_inc([0b0000_0001, 0b1111_1111], 15),
			(true, [0b0000_0001, 0b1111_1110]),
		);
		assert_eq!(
			u8_slice_inc([0b0000_0000, 0b0000_0000], 8),
			(false, [0b0000_0000, 0b0000_0001]),
		);
		assert_eq!(
			u8_slice_inc([0b0000_0000, 0b1111_1111], 8),
			(true, [0b0000_0000, 0b0000_0000]),
		);
		assert_eq!(
			u8_slice_inc([0b0000_0001, 0b1111_1111], 8),
			(true, [0b0000_0001, 0b0000_0000]),
		);
		assert_eq!(
			u8_slice_inc([0b0000_0000, 0b0000_0000], 0),
			(false, [0b0000_0000, 0b0000_0001]),
		);
		assert_eq!(
			u8_slice_inc([0b1111_1111, 0b1111_1111], 0),
			(true, [0b0000_0000, 0b0000_0000]),
		);
	}

	#[test]
	fn test_u8_slice_get() {
		assert!(!super::u8::slice_get(&[0, 0b0000_0000], 15));
		assert!(super::u8::slice_get(&[0, 0b0000_0001], 15));
		assert!(!super::u8::slice_get(&[0, 0b0000_0000], 14));
		assert!(super::u8::slice_get(&[0, 0b0000_0010], 14));
		assert!(!super::u8::slice_get(&[0, 0b0000_0000], 8));
		assert!(super::u8::slice_get(&[0, 0b1000_0000], 8));
		assert!(!super::u8::slice_get(&[0b0000_0000, 0], 7));
		assert!(super::u8::slice_get(&[0b0000_0001, 0], 7));
		assert!(!super::u8::slice_get(&[0b0000_0000, 0], 1));
		assert!(super::u8::slice_get(&[0b0100_0000, 0], 1));
		assert!(!super::u8::slice_get(&[0b0000_0000, 0], 0));
		assert!(super::u8::slice_get(&[0b1000_0000, 0], 0));
	}

	#[test]
	fn test_u8_slice_set() {
		assert_eq!(u8_slice_set([0, 0b0000_0000], 15, true), [0, 0b0000_0001]);
		assert_eq!(u8_slice_set([0, 0b0000_0001], 15, false), [0, 0b0000_0000]);
		assert_eq!(u8_slice_set([!0, 0b0000_0001], 15, true), [!0, 0b0000_0001]);
		assert_eq!(u8_slice_set([!0, 0b0000_0000], 15, false), [!0, 0b0000_0000]);
		assert_eq!(u8_slice_set([0, 0b0000_0000], 14, true), [0, 0b0000_0010]);
		assert_eq!(u8_slice_set([0, 0b0000_0010], 14, false), [0, 0b0000_0000]);
		assert_eq!(u8_slice_set([!0, 0b1111_1111], 14, true), [!0, 0b1111_1111]);
		assert_eq!(u8_slice_set([0, 0b0000_0000], 8, true), [0, 0b1000_0000]);
		assert_eq!(u8_slice_set([0, 0b1000_0000], 8, false), [0, 0b0000_0000]);
		assert_eq!(u8_slice_set([!0, 0b1111_1111], 8, true), [!0, 0b1111_1111]);
		assert_eq!(u8_slice_set([0b0000_0000, 0], 7, true), [0b0000_0001, 0]);
		assert_eq!(u8_slice_set([0b0000_0001, 0], 7, false), [0b0000_0000, 0]);
		assert_eq!(u8_slice_set([0b0000_0001, !0], 7, true), [0b0000_0001, !0]);
		assert_eq!(u8_slice_set([0b0000_0000, 0], 0, true), [0b1000_0000, 0]);
		assert_eq!(u8_slice_set([0b1000_0000, 0], 0, false), [0b0000_0000, 0]);
		assert_eq!(u8_slice_set([0b1111_1111, !0], 0, true), [0b1111_1111, !0]);
	}

	#[test]
	fn test_u8_slice_flip() {
		assert_eq!(u8_slice_flip([0, 0b0000_0000], 15), [0, 0b0000_0001]);
		assert_eq!(u8_slice_flip([0, 0b0000_0001], 15), [0, 0b0000_0000]);
		assert_eq!(u8_slice_flip([0, 0b0000_0000], 8), [0, 0b1000_0000]);
		assert_eq!(u8_slice_flip([!0, 0b1111_1111], 8), [!0, 0b0111_1111]);
		assert_eq!(u8_slice_flip([0b0000_0000, 0], 7), [0b0000_0001, 0]);
		assert_eq!(u8_slice_flip([0b0000_0001, 0], 7), [0b0000_0000, 0]);
		assert_eq!(u8_slice_flip([0b0000_0000, !0], 0), [0b1000_0000, !0]);
		assert_eq!(u8_slice_flip([0b1111_1111, 0], 0), [0b0111_1111, 0]);
	}

	#[test]
	fn test_u8_slice_shared_prefix_len() {
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

	#[test]
	fn test_u8_slice_set_false_from() {
		assert_eq!(u8_slice_set_false_from([0, 0b1111_1111], 16), [0, 0b1111_1111]);
		assert_eq!(u8_slice_set_false_from([!0, 0b1111_1111], 16), [!0, 0b1111_1111]);
		assert_eq!(u8_slice_set_false_from([!0, 0b1111_1111], 15), [!0, 0b1111_1110]);
		assert_eq!(u8_slice_set_false_from([!0, 0b1111_1111], 9), [!0, 0b1000_0000]);
		assert_eq!(u8_slice_set_false_from([0, 0b1111_1111], 8), [0, 0b0000_0000]);
		assert_eq!(u8_slice_set_false_from([!0, 0b1111_1111], 8), [!0, 0b0000_0000]);
		assert_eq!(u8_slice_set_false_from([0b1111_1111, !0], 7), [0b1111_1110, 0]);
		assert_eq!(u8_slice_set_false_from([0b1111_1111, !0], 1), [0b1000_0000, 0]);
		assert_eq!(u8_slice_set_false_from([0b1111_1111, !0], 0), [0b0000_0000, 0]);
	}

	#[test]
	fn test_u8_slice_is_false_from() {
		assert!(super::u8::slice_is_false_from(&[0, 0b1111_1111], 16));
		assert!(super::u8::slice_is_false_from(&[!0, 0b1111_1111], 16));
		assert!(super::u8::slice_is_false_from(&[!0, 0b1111_1110], 15));
		assert!(super::u8::slice_is_false_from(&[!0, 0b1000_0000], 9));
		assert!(super::u8::slice_is_false_from(&[0, 0b0000_0000], 8));
		assert!(super::u8::slice_is_false_from(&[!0, 0b0000_0000], 8));
		assert!(super::u8::slice_is_false_from(&[0b1111_1110, 0], 7));
		assert!(super::u8::slice_is_false_from(&[0b1000_0000, 0], 1));
		assert!(super::u8::slice_is_false_from(&[0b0000_0000, 0], 0));
	}

	#[test]
	fn test_u8_slice_set_true_from() {
		assert_eq!(u8_slice_set_true_from([!0, 0b1111_1111], 16), [!0, 0b1111_1111]);
		assert_eq!(u8_slice_set_true_from([0, 0b0000_0000], 16), [0, 0b0000_0000]);
		assert_eq!(u8_slice_set_true_from([0, 0b0000_0000], 15), [0, 0b0000_0001]);
		assert_eq!(u8_slice_set_true_from([0, 0b0000_0000], 9), [0, 0b0111_1111]);
		assert_eq!(u8_slice_set_true_from([!0, 0b0000_0000], 8), [!0, 0b1111_1111]);
		assert_eq!(u8_slice_set_true_from([0, 0b0000_0000], 8), [0, 0b1111_1111]);
		assert_eq!(u8_slice_set_true_from([0b0000_0000, 0], 7), [0b0000_0001, !0]);
		assert_eq!(u8_slice_set_true_from([0b0000_0000, 0], 1), [0b0111_1111, !0]);
		assert_eq!(u8_slice_set_true_from([0b0000_0000, 0], 0), [0b1111_1111, !0]);
	}

	#[test]
	fn test_u8_slice_is_true_from() {
		assert!(super::u8::slice_is_true_from(&[!0, 0b1111_1111], 16));
		assert!(super::u8::slice_is_true_from(&[0, 0b0000_0000], 16));
		assert!(super::u8::slice_is_true_from(&[0, 0b0000_0001], 15));
		assert!(super::u8::slice_is_true_from(&[0, 0b0111_1111], 9));
		assert!(super::u8::slice_is_true_from(&[!0, 0b1111_1111], 8));
		assert!(super::u8::slice_is_true_from(&[0, 0b1111_1111], 8));
		assert!(super::u8::slice_is_true_from(&[0b0000_0001, !0], 7));
		assert!(super::u8::slice_is_true_from(&[0b0111_1111, !0], 1));
		assert!(super::u8::slice_is_true_from(&[0b1111_1111, !0], 0));
	}

	#[test]
	fn test_u8_slice_contains() {
		assert!(super::u8::slice_contains(&[0, 0], 0, &[!0, !0]));
		assert!(super::u8::slice_contains(&[0b0000_0000, 0], 1, &[0b0111_1111, !0]));
		assert!(super::u8::slice_contains(&[0b1111_1111, !0], 1, &[0b1000_0000, 0]));
		assert!(super::u8::slice_contains(&[0b0000_0000, 0], 7, &[0b0000_0001, !0]));
		assert!(super::u8::slice_contains(&[0b1111_1111, 0], 7, &[0b1111_1110, !0]));
		assert!(!super::u8::slice_contains(&[0b0000_0000, 0], 7, &[0b1000_0001, 0]));
		assert!(!super::u8::slice_contains(&[0b0000_0000, 0], 7, &[0b0000_1001, 0]));
		assert!(!super::u8::slice_contains(&[0b0111_1111, 0], 7, &[0b1111_1110, 0]));
		assert!(!super::u8::slice_contains(&[0b1111_1111, 0], 7, &[0b1111_0110, 0]));
		assert!(super::u8::slice_contains(&[0b0000_0000, 0], 8, &[0b0000_0000, !0]));
		assert!(super::u8::slice_contains(&[0b1111_1111, !0], 8, &[0b1111_1111, 0]));
		assert!(!super::u8::slice_contains(&[0b0000_0000, 0], 8, &[0b0000_0001, 0]));
		assert!(!super::u8::slice_contains(&[0b0000_0000, 0], 8, &[0b1000_0000, 0]));
		assert!(!super::u8::slice_contains(&[0b1111_1111, 0], 8, &[0b1111_1110, 0]));
		assert!(!super::u8::slice_contains(&[0b1111_1111, 0], 8, &[0b0111_1111, 0]));

		assert!(super::u8::slice_contains(&[0, 0b0000_0000], 9, &[0, 0b0111_1111]));
		assert!(super::u8::slice_contains(&[!0, 0b1111_1111], 9, &[!0, 0b1000_0000]));
		assert!(super::u8::slice_contains(&[0, 0b0000_0000], 15, &[0, 0b0000_0001]));
		assert!(super::u8::slice_contains(&[!0, 0b1111_1111], 15, &[!0, 0b1111_1110]));
		assert!(!super::u8::slice_contains(&[0, 0b0000_0000], 15, &[0, 0b1000_0001]));
		assert!(!super::u8::slice_contains(&[0, 0b0000_0000], 15, &[0, 0b0000_1001]));
		assert!(!super::u8::slice_contains(&[!0, 0b0111_1111], 15, &[!0, 0b1111_1110]));
		assert!(!super::u8::slice_contains(&[!0, 0b1111_1111], 15, &[!0, 0b1111_0110]));
		assert!(super::u8::slice_contains(&[0, 0b0000_0000], 16, &[0, 0b0000_0000]));
		assert!(super::u8::slice_contains(&[!0, 0b1111_1111], 16, &[!0, 0b1111_1111]));
		assert!(!super::u8::slice_contains(&[0, 0b0000_0000], 16, &[0, 0b0000_0001]));
		assert!(!super::u8::slice_contains(&[0, 0b0000_0000], 16, &[0, 0b1000_0000]));
		assert!(!super::u8::slice_contains(&[!0, 0b1111_1111], 16, &[!0, 0b1111_1110]));
		assert!(!super::u8::slice_contains(&[!0, 0b1111_1111], 16, &[!0, 0b0111_1111]));
	}
}
