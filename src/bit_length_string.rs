use std::cmp::{
	min,
	Ordering,
};

use crate::{
	bit_string::BitString,
	fixed_bit_string::FixedBitString,
};

/// Extend a
/// [`FixedBitString`](fixed_bit_string/trait.FixedBitString.html) to a
/// [`BitString`](bit_string/trait.BitString.html) by also storing a
/// length.
#[derive(Clone, Debug, Hash)]
// TODO: drop the PartialEq + Eq manual implementations; instead require
// the underyling type to implement sane semantics (i.e. not contain any
// data outside what is accessible through "FixedBitString")
#[allow(clippy::derived_hash_with_manual_eq)]
pub struct BitLengthString<W: FixedBitString> {
	/// underlying bit string with fixed length
	bits: W,
	/// current length of [`BitString`](trait.BitString.html); should
	/// not exceed [`W::len()`](trait.FixedBitString.html#tymethod.len).
	len: usize,
}

impl<W: FixedBitString> BitLengthString<W> {
	/// Create new dynamic-length bit string from fixed bit string and a
	/// length.
	///
	/// The bits in `bits` after `len` bits are set to false.
	///
	/// # Panics
	///
	/// Panics if `len > W::len()`.
	pub fn new(mut bits: W, len: usize) -> Self {
		assert!(len <= W::len());
		bits.set_false_from(len);
		BitLengthString { bits, len }
	}

	/// check whether another bit string `bits` is prefixed by `self`
	pub fn contains(&self, bits: &W) -> bool {
		self.bits.contains(self.len, bits)
	}

	/// get read access to the bits
	pub fn bits(&self) -> &W {
		&self.bits
	}

	/// length of bit string (same as
	/// [`BitString::len()`](bit_string/trait.BitString.html#tymethod.len))
	#[allow(clippy::len_without_is_empty)]
	pub fn len(&self) -> usize {
		self.len
	}
}

impl<W: FixedBitString> BitString for BitLengthString<W> {
	fn get(&self, ndx: usize) -> bool {
		assert!(ndx < self.len);
		self.bits.get(ndx)
	}

	fn set(&mut self, ndx: usize, bit: bool) {
		assert!(ndx < self.len);
		self.bits.set(ndx, bit);
	}

	fn flip(&mut self, ndx: usize) {
		assert!(ndx < self.len);
		self.bits.flip(ndx);
	}

	fn len(&self) -> usize {
		debug_assert!(self.len <= W::len());
		self.len
	}

	fn clip(&mut self, len: usize) {
		self.bits.set_false_from(len);
		self.len = min(self.len, len);
	}

	fn append(&mut self, bit: bool) {
		self.bits.set(self.len, bit);
		self.len += 1;
	}

	fn null() -> Self {
		BitLengthString {
			bits: W::new_all_false(),
			len: 0,
		}
	}

	fn shared_prefix_len(&self, other: &Self) -> usize {
		let max_len = min(self.len, other.len);
		min(W::shared_prefix_len(&self.bits, &other.bits), max_len)
	}
}

impl<W: FixedBitString> Default for BitLengthString<W> {
	fn default() -> Self {
		Self::null()
	}
}

impl<W: FixedBitString> Ord for BitLengthString<W> {
	fn cmp(&self, rhs: &Self) -> Ordering {
		self.lexicographic_cmp(rhs)
	}
}
impl<W: FixedBitString> PartialOrd for BitLengthString<W> {
	fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
		Some(self.cmp(rhs))
	}
}

// TODO: we derive `Hash`, so we probably could derive this too
impl<W: FixedBitString> PartialEq for BitLengthString<W> {
	fn eq(&self, rhs: &Self) -> bool {
		Ordering::Equal == self.cmp(rhs)
	}
}
impl<W: FixedBitString> Eq for BitLengthString<W> {}
