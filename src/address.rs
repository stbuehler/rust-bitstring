use std::net::{
	Ipv4Addr,
	Ipv6Addr,
};

use crate::{
	fixed_bit_string::FixedBitString,
	utils::BigEndianBitString,
};

fn with_ipv4_mut_slice<F, T>(addr: &mut Ipv4Addr, f: F) -> T
where
	F: FnOnce(&mut [u8]) -> T,
{
	let mut o = addr.octets();
	let result = f(&mut o);
	*addr = Ipv4Addr::from(o);
	result
}

impl FixedBitString for Ipv4Addr {
	fn inc(&mut self, prefix: usize) -> bool {
		with_ipv4_mut_slice(self, |slice| BigEndianBitString::inc(slice, prefix))
	}

	fn len() -> usize {
		32
	}

	fn get(&self, ndx: usize) -> bool {
		BigEndianBitString::get(&self.octets(), ndx)
	}

	fn set(&mut self, ndx: usize, bit: bool) {
		with_ipv4_mut_slice(self, |slice| BigEndianBitString::set(slice, ndx, bit))
	}

	fn flip(&mut self, ndx: usize) {
		with_ipv4_mut_slice(self, |slice| BigEndianBitString::flip(slice, ndx))
	}

	fn shared_prefix_len(&self, other: &Self, max_len: usize) -> usize {
		BigEndianBitString::shared_prefix_len(&self.octets(), &other.octets(), max_len)
	}

	fn set_false_from(&mut self, ndx: usize) {
		with_ipv4_mut_slice(self, |slice| BigEndianBitString::set_false_from(slice, ndx))
	}

	fn is_false_from(&self, ndx: usize) -> bool {
		BigEndianBitString::is_false_from(&self.octets(), ndx)
	}

	fn set_true_from(&mut self, ndx: usize) {
		with_ipv4_mut_slice(self, |slice| BigEndianBitString::set_true_from(slice, ndx))
	}

	fn is_true_from(&self, ndx: usize) -> bool {
		BigEndianBitString::is_true_from(&self.octets(), ndx)
	}

	fn new_all_false() -> Self {
		Ipv4Addr::new(0, 0, 0, 0)
	}

	fn new_all_true() -> Self {
		Ipv4Addr::new(!0, !0, !0, !0)
	}

	fn contains(&self, prefix: usize, other: &Self) -> bool {
		BigEndianBitString::contains(&self.octets(), prefix, &other.octets())
	}
}

fn with_ipv6_mut_slice<F, T>(addr: &mut Ipv6Addr, f: F) -> T
where
	F: FnOnce(&mut [u8]) -> T,
{
	let mut o = addr.octets();
	let result = f(&mut o);
	*addr = Ipv6Addr::from(o);
	result
}

impl FixedBitString for Ipv6Addr {
	fn inc(&mut self, prefix: usize) -> bool {
		with_ipv6_mut_slice(self, |slice| BigEndianBitString::inc(slice, prefix))
	}

	fn len() -> usize {
		128
	}

	fn get(&self, ndx: usize) -> bool {
		BigEndianBitString::get(&self.octets(), ndx)
	}

	fn set(&mut self, ndx: usize, bit: bool) {
		with_ipv6_mut_slice(self, |slice| BigEndianBitString::set(slice, ndx, bit))
	}

	fn flip(&mut self, ndx: usize) {
		with_ipv6_mut_slice(self, |slice| BigEndianBitString::flip(slice, ndx))
	}

	fn shared_prefix_len(&self, other: &Self, max_len: usize) -> usize {
		BigEndianBitString::shared_prefix_len(&self.octets(), &other.octets(), max_len)
	}

	fn set_false_from(&mut self, ndx: usize) {
		with_ipv6_mut_slice(self, |slice| BigEndianBitString::set_false_from(slice, ndx))
	}

	fn is_false_from(&self, ndx: usize) -> bool {
		BigEndianBitString::is_false_from(&self.octets(), ndx)
	}

	fn set_true_from(&mut self, ndx: usize) {
		with_ipv6_mut_slice(self, |slice| BigEndianBitString::set_true_from(slice, ndx))
	}

	fn is_true_from(&self, ndx: usize) -> bool {
		BigEndianBitString::is_true_from(&self.octets(), ndx)
	}

	fn new_all_false() -> Self {
		Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)
	}

	fn new_all_true() -> Self {
		Ipv6Addr::new(!0, !0, !0, !0, !0, !0, !0, !0)
	}

	fn contains(&self, prefix: usize, other: &Self) -> bool {
		BigEndianBitString::contains(&self.octets(), prefix, &other.octets())
	}
}
