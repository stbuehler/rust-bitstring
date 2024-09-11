macro_rules! impl_big_endian_for {
	($mod:ident => $t:ty) => {
		/// `BigEndianBitString` functions for specific integer type
		#[cfg_attr(not(feature = "bigendian"), allow(dead_code))] // lots of unused parts unless we export it
		pub mod $mod {
			use core::cmp::min;

			/// bits in a single element
			pub const ELEMENT_BITS: usize = <$t>::BITS as usize;

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
			pub const fn make_element_inc(value: $t, prefix: usize) -> ($t, bool) {
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
			/// Panics if `prefix > ELEMENT_BITS`.
			pub fn element_inc(value: &mut $t, prefix: usize) -> bool {
				let overflow;
				(*value, overflow) = make_element_inc(*value, prefix);
				overflow
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

				element_inc(&mut slice[slice_ndx], element_ndx)
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
			pub const fn make_element_set(value: $t, ndx: usize, bit: bool) -> $t {
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
			/// Panics if `ndx >= ELEMENT_BITS`.
			pub fn element_set(value: &mut $t, ndx: usize, bit: bool) {
				*value = make_element_set(*value, ndx, bit);
			}

			/// Set the `ndx`th bit to `bit`.
			///
			/// # Panics
			///
			/// Panics if `ndx >= ELEMENT_BITS * slice.len()`.
			pub fn slice_set(slice: &mut [$t], ndx: usize, bit: bool) {
				let slice_ndx = ndx / ELEMENT_BITS;
				element_set(&mut slice[slice_ndx], ndx % ELEMENT_BITS, bit);
			}

			/// Flips the `ndx`th bit.
			///
			/// # Panics
			///
			/// Panics if `ndx >= ELEMENT_BITS`.
			pub const fn make_element_flip(value: $t, ndx: usize) -> $t {
				assert!(ndx < ELEMENT_BITS);
				value ^ mask(ndx)
			}

			/// Flips the `ndx`th bit.
			///
			/// # Panics
			///
			/// Panics if `ndx >= ELEMENT_BITS`.
			pub fn element_flip(value: &mut $t, ndx: usize) {
				*value = make_element_flip(*value, ndx);
			}

			/// Flips the `ndx`th bit.
			///
			/// # Panics
			///
			/// Panics if `ndx >= ELEMENT_BITS * slice.len()`.
			pub fn slice_flip(slice: &mut [$t], ndx: usize) {
				let slice_ndx = ndx / ELEMENT_BITS;
				element_flip(&mut slice[slice_ndx], ndx % ELEMENT_BITS);
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
			pub const fn make_element_set_false_from(value: $t, ndx: usize) -> $t {
				if ndx >= ELEMENT_BITS {
					return value;
				}
				value & !mask_suffix(ndx)
			}

			/// Set all bits from [ndx..] to `false` (`0`).
			///
			/// Doesn't do anything if `ndx >= ELEMENT_BITS`.
			pub fn element_set_false_from(value: &mut $t, ndx: usize) {
				*value = make_element_set_false_from(*value, ndx);
			}

			/// Set all bits from [ndx..] to `false` (`0`).
			///
			/// Doesn't do anything if `ndx >= ELEMENT_BITS * slice.len()`.
			pub fn slice_set_false_from(slice: &mut [$t], ndx: usize) {
				let slice_ndx = ndx / ELEMENT_BITS;
				if slice_ndx >= slice.len() {
					return;
				}
				element_set_false_from(&mut slice[slice_ndx], ndx % ELEMENT_BITS);
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
			pub const fn make_element_set_true_from(value: $t, ndx: usize) -> $t {
				if ndx >= ELEMENT_BITS {
					return value;
				}
				value | mask_suffix(ndx)
			}

			/// Set all bits from [ndx..] to `true` (`1`).
			///
			/// Doesn't do anything if `ndx >= ELEMENT_BITS`.
			pub fn element_set_true_from(value: &mut $t, ndx: usize) {
				*value = make_element_set_true_from(*value, ndx);
			}

			/// Set all bits from [ndx..] to `true` (`1`).
			///
			/// Doesn't do anything if `ndx >= ELEMENT_BITS * slice.len()`.
			pub fn slice_set_true_from(slice: &mut [$t], ndx: usize) {
				let slice_ndx = ndx / ELEMENT_BITS;
				if slice_ndx >= slice.len() {
					return;
				}
				element_set_true_from(&mut slice[slice_ndx], ndx % ELEMENT_BITS);
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
	};
}

impl_big_endian_for! {u8 => u8}
impl_big_endian_for! {u16 => u16}
impl_big_endian_for! {u32 => u32}
impl_big_endian_for! {u64 => u64}
impl_big_endian_for! {u128 => u128}
