use core::net::{
	Ipv4Addr,
	Ipv6Addr,
};

use crate::{
	FixedBitString,
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

fn with_ipv4_mut_u32<F, T>(addr: &mut Ipv4Addr, f: F) -> T
where
	F: FnOnce(&mut u32) -> T,
{
	let mut o = addr.to_bits();
	let result = f(&mut o);
	*addr = Ipv4Addr::from_bits(o);
	result
}

impl FixedBitString for Ipv4Addr {
	fn inc(&mut self, prefix: usize) -> bool {
		with_ipv4_mut_u32(self, |num| num.bits_inc(prefix))
	}

	fn len() -> usize {
		32
	}

	fn get(&self, ndx: usize) -> bool {
		self.octets().as_slice().bit_get(ndx)
	}

	fn set(&mut self, ndx: usize, bit: bool) {
		with_ipv4_mut_slice(self, |slice| slice.bit_set(ndx, bit))
	}

	fn flip(&mut self, ndx: usize) {
		with_ipv4_mut_slice(self, |slice| slice.bit_flip(ndx))
	}

	fn shared_prefix_len(&self, other: &Self) -> usize {
		self.to_bits()
			.shared_prefix_len(&other.to_bits(), Self::len())
	}

	fn set_false_from(&mut self, ndx: usize) {
		with_ipv4_mut_u32(self, |num| num.set_false_from(ndx))
	}

	fn is_false_from(&self, ndx: usize) -> bool {
		self.to_bits().is_false_from(ndx)
	}

	fn set_true_from(&mut self, ndx: usize) {
		with_ipv4_mut_u32(self, |num| num.set_true_from(ndx))
	}

	fn is_true_from(&self, ndx: usize) -> bool {
		self.to_bits().is_true_from(ndx)
	}

	fn new_all_false() -> Self {
		Ipv4Addr::from_bits(0)
	}

	fn new_all_true() -> Self {
		Ipv4Addr::from_bits(!0)
	}

	fn contains(&self, prefix: usize, other: &Self) -> bool {
		self.to_bits().bits_prefix_of(prefix, &other.to_bits())
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

fn with_ipv6_mut_u128<F, T>(addr: &mut Ipv6Addr, f: F) -> T
where
	F: FnOnce(&mut u128) -> T,
{
	let mut o = addr.to_bits();
	let result = f(&mut o);
	*addr = Ipv6Addr::from_bits(o);
	result
}

impl FixedBitString for Ipv6Addr {
	fn inc(&mut self, prefix: usize) -> bool {
		with_ipv6_mut_slice(self, |slice| slice.bits_inc(prefix))
	}

	fn len() -> usize {
		128
	}

	fn get(&self, ndx: usize) -> bool {
		self.octets().as_slice().bit_get(ndx)
	}

	fn set(&mut self, ndx: usize, bit: bool) {
		with_ipv6_mut_slice(self, |slice| slice.bit_set(ndx, bit))
	}

	fn flip(&mut self, ndx: usize) {
		with_ipv6_mut_slice(self, |slice| slice.bit_flip(ndx))
	}

	fn shared_prefix_len(&self, other: &Self) -> usize {
		self.to_bits()
			.shared_prefix_len(&other.to_bits(), Self::len())
	}

	fn set_false_from(&mut self, ndx: usize) {
		with_ipv6_mut_u128(self, |num| num.set_false_from(ndx))
	}

	fn is_false_from(&self, ndx: usize) -> bool {
		self.to_bits().is_false_from(ndx)
	}

	fn set_true_from(&mut self, ndx: usize) {
		with_ipv6_mut_u128(self, |num| num.set_true_from(ndx))
	}

	fn is_true_from(&self, ndx: usize) -> bool {
		self.to_bits().is_true_from(ndx)
	}

	fn new_all_false() -> Self {
		Ipv6Addr::from_bits(0)
	}

	fn new_all_true() -> Self {
		Ipv6Addr::from_bits(!0)
	}

	fn contains(&self, prefix: usize, other: &Self) -> bool {
		self.to_bits().bits_prefix_of(prefix, &other.to_bits())
	}
}
