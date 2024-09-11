use super::traits::BigEndianBitString;

use super::int_helpers::{
	u128,
	u16,
	u32,
	u64,
	u8,
};

macro_rules! impl_big_endian_for {
	($mod:ident => $t:ty) => {
		impl BigEndianBitString for $t {
			fn bits(&self) -> usize {
				$mod::ELEMENT_BITS
			}

			fn bits_inc(&mut self, prefix: usize) -> bool {
				let overflow;
				(*self, overflow) = $mod::element_inc(*self, prefix);
				overflow
			}

			fn bit_get(&self, ndx: usize) -> bool {
				$mod::element_get(*self, ndx)
			}

			fn bit_set(&mut self, ndx: usize, bit: bool) {
				*self = $mod::element_set(*self, ndx, bit)
			}

			fn bit_flip(&mut self, ndx: usize) {
				*self = $mod::element_flip(*self, ndx)
			}

			fn shared_prefix_len(&self, other: &Self, max_len: usize) -> usize {
				$mod::element_shared_prefix_len(*self, *other, max_len)
			}

			fn set_false_from(&mut self, ndx: usize) {
				*self = $mod::element_set_false_from(*self, ndx);
			}

			fn is_false_from(&self, ndx: usize) -> bool {
				$mod::element_is_false_from(*self, ndx)
			}

			fn set_true_from(&mut self, ndx: usize) {
				*self = $mod::element_set_true_from(*self, ndx);
			}

			fn is_true_from(&self, ndx: usize) -> bool {
				$mod::element_is_true_from(*self, ndx)
			}

			fn bits_prefix_of(&self, prefix: usize, other: &Self) -> bool {
				$mod::element_contains(*self, prefix, *other)
			}
		}

		impl BigEndianBitString for [$t] {
			fn bits(&self) -> usize {
				self.len() * $mod::ELEMENT_BITS
			}

			fn bits_inc(&mut self, prefix: usize) -> bool {
				$mod::slice_inc(self, prefix)
			}

			fn bit_get(&self, ndx: usize) -> bool {
				$mod::slice_get(self, ndx)
			}

			fn bit_set(&mut self, ndx: usize, bit: bool) {
				$mod::slice_set(self, ndx, bit)
			}

			fn bit_flip(&mut self, ndx: usize) {
				$mod::slice_flip(self, ndx)
			}

			fn shared_prefix_len(&self, other: &Self, max_len: usize) -> usize {
				$mod::slice_shared_prefix_len(self, other, max_len)
			}

			fn set_false_from(&mut self, ndx: usize) {
				$mod::slice_set_false_from(self, ndx)
			}

			fn is_false_from(&self, ndx: usize) -> bool {
				$mod::slice_is_false_from(self, ndx)
			}

			fn set_true_from(&mut self, ndx: usize) {
				$mod::slice_set_true_from(self, ndx)
			}

			fn is_true_from(&self, ndx: usize) -> bool {
				$mod::slice_is_true_from(self, ndx)
			}

			fn bits_prefix_of(&self, prefix: usize, other: &Self) -> bool {
				$mod::slice_contains(self, prefix, other)
			}
		}
	};
}

impl_big_endian_for! {u8 => u8}
impl_big_endian_for! {u16 => u16}
impl_big_endian_for! {u32 => u32}
impl_big_endian_for! {u64 => u64}
impl_big_endian_for! {u128 => u128}
