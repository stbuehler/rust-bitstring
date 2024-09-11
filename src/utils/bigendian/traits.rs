/// Generic helper methods to treat unsigned integer and slices of them as big endian bit strings.
pub trait BigEndianBitString {
	/// maximum number of bits in storage
	fn bits(&self) -> usize;

	/// increment from right; don't touch first `prefix` bits; returns
	/// true on overflow
	///
	/// # Panics
	///
	/// Panics if `prefix > self.bits()`.
	fn bits_inc(&mut self, prefix: usize) -> bool;

	/// Get the `ndx`th bit.
	///
	/// # Panics
	///
	/// Panics if `ndx >= Self::ELEMENT_BITS() * slice.len()`.
	fn bit_get(&self, ndx: usize) -> bool;

	/// Set the `ndx`th bit to `bit`.
	///
	/// # Panics
	///
	/// Panics if `ndx >= Self::ELEMENT_BITS() * slice.len()`.
	fn bit_set(&mut self, ndx: usize, bit: bool);

	/// Flips the `ndx`th bit.
	///
	/// # Panics
	///
	/// Panics if `ndx >= Self::ELEMENT_BITS() * slice.len()`.
	fn bit_flip(&mut self, ndx: usize);

	/// Length of the longest shared prefix of two bit strings.
	fn shared_prefix_len(&self, other: &Self, max_len: usize) -> usize;

	/// Set all bits from [ndx..] to `false` (`0`).
	///
	/// Doesn't do anything if `ndx >= Self::ELEMENT_BITS() * slice.len()`.
	fn set_false_from(&mut self, ndx: usize);

	/// Whether all bits from [ndx..] are `false` (`0`).
	///
	/// Returns `true` if `ndx >= Self::ELEMENT_BITS() * slice.len()`.
	fn is_false_from(&self, ndx: usize) -> bool;

	/// Set all bits from [ndx..] to `true` (`1`).
	///
	/// Doesn't do anything if `ndx >= Self::ELEMENT_BITS() * slice.len()`.
	fn set_true_from(&mut self, ndx: usize);

	/// Whether all bits from [ndx..] are `true` (`1`).
	///
	/// Returns `true` if `ndx >= Self::ELEMENT_BITS() * slice.len()`.
	fn is_true_from(&self, ndx: usize) -> bool;

	/// check whether another bit string `value` shares the first
	/// `prefix_len` bits with `self`
	fn bits_prefix_of(&self, prefix_len: usize, value: &Self) -> bool;
}
