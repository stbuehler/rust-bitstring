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
