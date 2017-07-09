use std::cmp::min;

use fixed_bit_string::Iter;

/// A bit string with fixed length.
///
/// All bits must me mutable, and there must be no dependencies between
/// bits (i.e. setting one bit must not change any other bit).
pub trait FixedBitString {
	/// Treat bit string as integer, where bit 0 is the most significant
	/// bit.
	///
	/// Increment by one, i.e. start by incrementing the bit with the
	/// highest index.
	///
	/// Don't touch first `prefix` bits; return true on overflow.
	///
	/// # Panics
	///
	/// Should panic if `prefix > self.len()`.
	fn inc(&mut self, prefix: usize) -> bool;

	/// Iterate through all bit strings until `inc` overflows.
	///
	/// All generated values will share the first `prefix` bits.  If you
	/// want to iterate over all values make sure to call
	/// `self.set_false_from(prefix)` before.
	///
	/// # Panics
	///
	/// Should panic if `prefix > self.len()`.
	fn iter(&self, prefix: usize) -> Iter<Self>
	where Self: Sized+Clone {
		Iter::new(self.clone(), prefix)
	}

	/// Length of the bit string in bits.
	fn len() -> usize;

	/// Get the `ndx`th bit.
	///
	/// # Panics
	///
	/// Should panic if `ndx >= self.len()`.
	fn get(&self, ndx: usize) -> bool;

	/// Set the `ndx`th bit to `bit`.
	///
	/// # Panics
	///
	/// Should panic if `ndx >= self.len()`.
	fn set(&mut self, ndx: usize, bit: bool);

	/// Set the `ndx`th bit to `true`.
	///
	/// # Panics
	///
	/// Should panic if `ndx >= self.len()`.
	fn on(&mut self, ndx: usize) {
		self.set(ndx, true);
	}

	/// Set the `ndx`th bit to `false`.
	///
	/// # Panics
	///
	/// Should panic if `ndx >= self.len()`.
	fn off(&mut self, ndx: usize) {
		self.set(ndx, false);
	}

	/// Flips the `ndx`th bit.
	///
	/// # Panics
	///
	/// Should panic if `ndx >= self.len()`.
	fn flip(&mut self, ndx: usize) {
		let old_value = self.get(ndx);
		self.set(ndx, !old_value);
	}

	/// Length of the longest shared prefix of two bit strings.
	fn shared_prefix_len(&self, other: &Self, max_len: usize) -> usize {
		let max_len = min(max_len, Self::len());
		for i in 0..max_len {
			if self.get(i) != other.get(i) {
				return i
			}
		}
		max_len
	}

	/// Set all bits in [ndx..] to `false`.
	///
	/// Doesn't do anything if `ndx >= self.len()`.
	fn set_false_from(&mut self, ndx: usize);

	/// Whether all bits in [ndx..] are `false`.
	///
	/// Returns `true` if `ndx >= self.len()`.
	fn is_false_from(&self, ndx: usize) -> bool;

	/// Set all bits in [ndx..] to `true`.
	///
	/// Doesn't do anything if `ndx >= self.len()`.
	fn set_true_from(&mut self, ndx: usize);

	/// Whether all bits in [ndx..] are `true`.
	///
	/// Returns `true` if `ndx >= self.len()`.
	fn is_true_from(&self, ndx: usize) -> bool;

	/// New bit string with all bits set to `false`.
	fn new_all_false() -> Self;

	/// New bit string with all bits set to `true`.
	fn new_all_true() -> Self;

	/// check whether another bit string `other` shares the first
	/// `prefix` bits with `self`
	fn contains(&self, prefix: usize, other: &Self) -> bool;
}
