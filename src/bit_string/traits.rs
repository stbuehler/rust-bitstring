use core::cmp::{
	min,
	Ordering,
};

/// A bit string with variable (but possibly limited) length.
///
/// The length limit might depend on the current string; that is why
/// writing a bit might truncate the string (but not the bit that was
/// just modified).  "Writing" a bit also includes writing without
/// actually changing it.
///
/// This special case is needed to handle variants with different
/// (maximum) lengths: a small prefix indicates the variant, then
/// follows the actual data of the variant.
///
/// As an example one might want to combine IPv4 and IPv6 cidr
/// representations into one `BitString` type; the empty bit string
/// would represent `0.0.0.0/0` + `::/0`.
///
/// Apart from this special case writing a bit must not modify any other
/// bits or change the length.
///
/// The required `Eq` implementation must match comparing two bitstrings
/// by their bits (up to their length); i.e. `BitString`s must not carry
/// additional data apart from the bits (and mustn't compare unused
/// bits in the storage if their value isn't fixed).
#[allow(clippy::len_without_is_empty)]
pub trait BitString: Eq {
	/// Get the `ndx`th bit.
	///
	/// # Panics
	///
	/// Should panic if `ndx >= self.len()`.
	fn get(&self, ndx: usize) -> bool;

	/// Set the `ndx`th bit to `bit`.
	///
	/// Might clip the length to `ndx+1`.
	///
	/// # Panics
	///
	/// Should panic if `ndx >= self.len()`.
	fn set(&mut self, ndx: usize, bit: bool);

	/// Flips the `ndx`th bit.
	///
	/// # Panics
	///
	/// Should panic if `ndx >= self.len()`.
	fn flip(&mut self, ndx: usize);

	/// Current length of the bit string in bits.
	#[allow(clippy::len_without_is_empty)]
	fn len(&self) -> usize;

	/// Set current length to `len`.
	///
	/// Does nothing if `len <= self.len()`.
	///
	/// If necessary should also zero the underlying storage if `Eq`
	/// needs it to work properly.
	fn clip(&mut self, len: usize);

	/// Append a bit.
	///
	/// # Panics
	///
	/// Might panic if underlying storage can only store a limited
	/// number of bits.
	fn append(&mut self, bit: bool);

	/// Create a new zero-length bit string.
	///
	/// Underlying storage should be zeroed if `Eq` needs it to work
	/// properly.
	fn null() -> Self;

	/// Length of the longest shared prefix of two bit strings.
	fn shared_prefix_len(&self, other: &Self) -> usize {
		let max_len = min(self.len(), other.len());
		for i in 0..max_len {
			if self.get(i) != other.get(i) {
				return i;
			}
		}
		max_len
	}

	/// Longest shared prefix of two bit strings.
	fn shared_prefix(&self, other: &Self) -> Self
	where
		Self: Clone,
	{
		let mut a = self.clone();
		a.clip(self.shared_prefix_len(other));
		a
	}

	/// Partial ordering on bit strings.
	///
	/// Formal definition:
	///
	/// ```text
	///     `a < b` iff `a != b` and `b` is a prefix of `a`
	/// ```
	///
	/// If you view a bit string as a set including all bit strings
	/// starting with it, this is the subset relation.
	fn subset_cmp(&self, other: &Self) -> Option<Ordering> {
		let spl = self.shared_prefix_len(other);
		if spl == self.len() {
			// self is a prefix of other
			if spl == other.len() {
				Some(Ordering::Equal)
			} else {
				Some(Ordering::Greater)
			}
		} else if spl == other.len() {
			// other is a prefix of self
			Some(Ordering::Less)
		} else {
			// neither is a prefix of the other one
			None
		}
	}

	/// Lexicographic ordering on bit strings.
	///
	/// Formal definition:
	///
	/// ```text
	///     `a < b` iff `a != b` and (
	///         `b` is a prefix of `a`
	///         or `a[s] < b[s]`
	///              where s is the smallest index with `a[s] != b[s]`)
	/// ```
	///
	/// Or, if you define `_|_ < false < true`:
	///
	/// ```text
	///     `a < b` iff `a[s] < b[s]`
	///         where s is the smallest index with `a[s] != b[s]`
	/// ```
	fn lexicographic_cmp(&self, other: &Self) -> Ordering {
		let spl = self.shared_prefix_len(other);
		if spl == self.len() {
			// self is a prefix of other
			if spl == other.len() {
				Ordering::Equal
			} else {
				// self is shorter than other
				Ordering::Less
			}
		} else if spl == other.len() {
			// other is a prefix of self and shorter
			Ordering::Greater
		} else {
			// both are at least one bit longer than the shared prefix,
			// and they differ in that bit (otherwise shared prefix
			// would be longer)
			if self.get(spl) {
				Ordering::Greater
			} else {
				Ordering::Less
			}
		}
	}
}
