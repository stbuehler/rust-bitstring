use crate::fixed_bit_string::FixedBitString;

/// Iterator to iterate over all
/// [`FixedBitString`](trait.FixedBitString.html) values with a shared
/// prefix.
///
/// Created by
/// [`FixedBitString::iter()`](trait.FixedBitString.html#method.iter).
#[derive(Clone, Debug)]
pub struct Iter<B> {
	next: Option<B>,
	prefix: usize,
}

impl<B: FixedBitString> Iter<B> {
	#[doc(hidden)]
	// internal use only, will become pub(crate)
	pub fn new(start: B, prefix: usize) -> Self {
		Iter {
			next: Some(start),
			prefix,
		}
	}
}

impl<B: FixedBitString + Clone> Iterator for Iter<B> {
	type Item = B;

	fn next(&mut self) -> Option<Self::Item> {
		let mut overflow = false;
		let result = match self.next {
			None => None,
			Some(ref mut next) => {
				let result = Some(next.clone());
				overflow = next.inc(self.prefix);
				result
			},
		};
		if overflow {
			self.next = None;
		}
		result
	}
}
