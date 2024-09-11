use core::net::{
	Ipv4Addr,
	Ipv6Addr,
};

use crate::{
	utils::bigendian::int_helpers::{
		u128,
		u32,
		u8,
	},
	FixedBitString,
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
		with_ipv4_mut_u32(self, |num| u32::element_inc(num, prefix))
	}

	fn len() -> usize {
		32
	}

	fn get(&self, ndx: usize) -> bool {
		u8::slice_get(&self.octets(), ndx)
	}

	fn set(&mut self, ndx: usize, bit: bool) {
		with_ipv4_mut_slice(self, |slice| u8::slice_set(slice, ndx, bit))
	}

	fn flip(&mut self, ndx: usize) {
		with_ipv4_mut_slice(self, |slice| u8::slice_flip(slice, ndx))
	}

	fn shared_prefix_len(&self, other: &Self) -> usize {
		u32::element_shared_prefix_len(self.to_bits(), other.to_bits(), Self::len())
	}

	fn set_false_from(&mut self, ndx: usize) {
		with_ipv4_mut_u32(self, |num| u32::element_set_false_from(num, ndx))
	}

	fn is_false_from(&self, ndx: usize) -> bool {
		u32::element_is_false_from(self.to_bits(), ndx)
	}

	fn set_true_from(&mut self, ndx: usize) {
		with_ipv4_mut_u32(self, |num| u32::element_set_true_from(num, ndx))
	}

	fn is_true_from(&self, ndx: usize) -> bool {
		u32::element_is_true_from(self.to_bits(), ndx)
	}

	fn new_all_false() -> Self {
		Ipv4Addr::from_bits(0)
	}

	fn new_all_true() -> Self {
		Ipv4Addr::from_bits(!0)
	}

	fn contains(&self, prefix: usize, other: &Self) -> bool {
		u32::element_contains(self.to_bits(), prefix, other.to_bits())
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
		with_ipv6_mut_u128(self, |num| u128::element_inc(num, prefix))
	}

	fn len() -> usize {
		128
	}

	fn get(&self, ndx: usize) -> bool {
		u8::slice_get(&self.octets(), ndx)
	}

	fn set(&mut self, ndx: usize, bit: bool) {
		with_ipv6_mut_slice(self, |slice| u8::slice_set(slice, ndx, bit))
	}

	fn flip(&mut self, ndx: usize) {
		with_ipv6_mut_slice(self, |slice| u8::slice_flip(slice, ndx))
	}

	fn shared_prefix_len(&self, other: &Self) -> usize {
		u128::element_shared_prefix_len(self.to_bits(), other.to_bits(), Self::len())
	}

	fn set_false_from(&mut self, ndx: usize) {
		with_ipv6_mut_u128(self, |num| u128::element_set_false_from(num, ndx))
	}

	fn is_false_from(&self, ndx: usize) -> bool {
		u128::element_is_false_from(self.to_bits(), ndx)
	}

	fn set_true_from(&mut self, ndx: usize) {
		with_ipv6_mut_u128(self, |num| u128::element_set_true_from(num, ndx))
	}

	fn is_true_from(&self, ndx: usize) -> bool {
		u128::element_is_true_from(self.to_bits(), ndx)
	}

	fn new_all_false() -> Self {
		Ipv6Addr::from_bits(0)
	}

	fn new_all_true() -> Self {
		Ipv6Addr::from_bits(!0)
	}

	fn contains(&self, prefix: usize, other: &Self) -> bool {
		u128::element_contains(self.to_bits(), prefix, other.to_bits())
	}
}
