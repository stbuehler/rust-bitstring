use super::int_helpers::u8;

fn u8_slice_inc<S: AsMut<[u8]>>(mut slice: S, prefix: usize) -> (bool, S) {
	let overflow = u8::slice_inc(slice.as_mut(), prefix);
	(overflow, slice)
}

fn u8_slice_set<S: AsMut<[u8]>>(mut slice: S, ndx: usize, bit: bool) -> S {
	u8::slice_set(slice.as_mut(), ndx, bit);
	slice
}

fn u8_slice_flip<S: AsMut<[u8]>>(mut slice: S, ndx: usize) -> S {
	u8::slice_flip(slice.as_mut(), ndx);
	slice
}

fn u8_slice_set_false_from<S: AsMut<[u8]>>(mut slice: S, ndx: usize) -> S {
	u8::slice_set_false_from(slice.as_mut(), ndx);
	slice
}

fn u8_slice_set_true_from<S: AsMut<[u8]>>(mut slice: S, ndx: usize) -> S {
	u8::slice_set_true_from(slice.as_mut(), ndx);
	slice
}

#[test]
fn test_u8_element_inc() {
	assert_eq!(u8::element_inc(0b0000_0000, 0), (0b0000_0001, false));
	assert_eq!(u8::element_inc(0b0000_0000, 4), (0b0000_0001, false));
	assert_eq!(u8::element_inc(0b0000_0000, 8), (0b0000_0000, true));
	assert_eq!(u8::element_inc(0b0000_1000, 0), (0b0000_1001, false));
	assert_eq!(u8::element_inc(0b0000_1000, 4), (0b0000_1001, false));
	assert_eq!(u8::element_inc(0b0000_1000, 8), (0b0000_1000, true));
	assert_eq!(u8::element_inc(0b0000_1111, 0), (0b0001_0000, false));
	assert_eq!(u8::element_inc(0b0000_1111, 4), (0b0000_0000, true));
	assert_eq!(u8::element_inc(0b0000_1111, 8), (0b0000_1111, true));
	assert_eq!(u8::element_inc(0b0001_1111, 0), (0b0010_0000, false));
	assert_eq!(u8::element_inc(0b0001_1111, 4), (0b0001_0000, true));
	assert_eq!(u8::element_inc(0b0001_1111, 8), (0b0001_1111, true));
	assert_eq!(u8::element_inc(0b1111_1111, 0), (0b0000_0000, true));
	assert_eq!(u8::element_inc(0b1111_1111, 4), (0b1111_0000, true));
	assert_eq!(u8::element_inc(0b1111_1111, 8), (0b1111_1111, true));
}

#[test]
fn test_u8_element_set() {
	assert_eq!(u8::element_set(0b0000_0000, 0, true), 0b1000_0000);
	assert_eq!(u8::element_set(0b1000_0000, 0, true), 0b1000_0000);
	assert_eq!(u8::element_set(0b0000_0000, 4, true), 0b0000_1000);
	assert_eq!(u8::element_set(0b0000_1000, 4, true), 0b0000_1000);
	assert_eq!(u8::element_set(0b0000_0000, 7, true), 0b0000_0001);
	assert_eq!(u8::element_set(0b0000_0001, 7, true), 0b0000_0001);

	assert_eq!(u8::element_set(0b1000_0000, 0, false), 0b0000_0000);
	assert_eq!(u8::element_set(0b0000_0000, 0, false), 0b0000_0000);
	assert_eq!(u8::element_set(0b0000_1000, 4, false), 0b0000_0000);
	assert_eq!(u8::element_set(0b0000_0000, 4, false), 0b0000_0000);
	assert_eq!(u8::element_set(0b0000_0001, 7, false), 0b0000_0000);
	assert_eq!(u8::element_set(0b0000_0000, 7, false), 0b0000_0000);
}

#[test]
fn test_u8_element_flip() {
	assert_eq!(u8::element_flip(0b0000_0000, 0), 0b1000_0000);
	assert_eq!(u8::element_flip(0b1000_0000, 0), 0b0000_0000);
	assert_eq!(u8::element_flip(0b0000_0000, 4), 0b0000_1000);
	assert_eq!(u8::element_flip(0b0000_1000, 4), 0b0000_0000);
	assert_eq!(u8::element_flip(0b0000_0000, 7), 0b0000_0001);
	assert_eq!(u8::element_flip(0b0000_0001, 7), 0b0000_0000);
}

#[test]
fn test_u8_element_shared_prefix_len() {
	assert_eq!(
		u8::element_shared_prefix_len(0b0000_0000, 0b0000_0000, 0),
		0
	);
	assert_eq!(
		u8::element_shared_prefix_len(0b0000_0000, 0b1000_0000, 8),
		0
	);
	assert_eq!(
		u8::element_shared_prefix_len(0b0000_0000, 0b0000_0000, 1),
		1
	);
	assert_eq!(
		u8::element_shared_prefix_len(0b0000_0000, 0b0100_0000, 8),
		1
	);
	assert_eq!(
		u8::element_shared_prefix_len(0b1100_0000, 0b1100_0001, 7),
		7
	);
	assert_eq!(
		u8::element_shared_prefix_len(0b1100_0000, 0b1100_0001, 8),
		7
	);
	assert_eq!(
		u8::element_shared_prefix_len(0b0000_0000, 0b0000_0000, 8),
		8
	);
	assert_eq!(
		u8::element_shared_prefix_len(0b1100_0001, 0b1100_0001, 8),
		8
	);
	assert_eq!(
		u8::element_shared_prefix_len(0b1111_1111, 0b1111_1111, 8),
		8
	);
}

#[test]
fn test_u8_element_get() {
	assert_eq!(u8::element_get(0b0000_0000, 0), false);
	assert_eq!(u8::element_get(0b1000_0000, 0), true);
	assert_eq!(u8::element_get(0b0000_0000, 1), false);
	assert_eq!(u8::element_get(0b0100_0000, 1), true);
	assert_eq!(u8::element_get(0b0000_0000, 2), false);
	assert_eq!(u8::element_get(0b0010_0000, 2), true);
	assert_eq!(u8::element_get(0b0000_0000, 3), false);
	assert_eq!(u8::element_get(0b0001_0000, 3), true);
	assert_eq!(u8::element_get(0b0000_0000, 6), false);
	assert_eq!(u8::element_get(0b0000_0010, 6), true);
	assert_eq!(u8::element_get(0b0000_0000, 7), false);
	assert_eq!(u8::element_get(0b0000_0001, 7), true);
}

#[test]
fn test_u8_element_is_false_from() {
	assert!(u8::element_is_false_from(0b0000_0000, 0));
	assert!(!u8::element_is_false_from(0b1111_1111, 0));
	assert!(u8::element_is_false_from(0b0000_0000, 1));
	assert!(u8::element_is_false_from(0b1000_0000, 1));
	assert!(!u8::element_is_false_from(0b1111_1111, 1));
	assert!(u8::element_is_false_from(0b0001_0000, 4));
	assert!(u8::element_is_false_from(0b1111_0000, 4));
	assert!(!u8::element_is_false_from(0b0000_1111, 4));
	assert!(u8::element_is_false_from(0b1111_1110, 7));
	assert!(!u8::element_is_false_from(0b1111_1111, 7));
	assert!(u8::element_is_false_from(0b1111_1111, 8));
}

#[test]
fn test_u8_element_set_false_from() {
	assert_eq!(u8::element_set_false_from(0b0000_0000, 0), 0b0000_0000);
	assert_eq!(u8::element_set_false_from(0b1111_1111, 0), 0b0000_0000);
	assert_eq!(u8::element_set_false_from(0b1111_0000, 4), 0b1111_0000);
	assert_eq!(u8::element_set_false_from(0b1111_1111, 4), 0b1111_0000);
	assert_eq!(u8::element_set_false_from(0b0000_1111, 8), 0b0000_1111);
	assert_eq!(u8::element_set_false_from(0b1111_0000, 8), 0b1111_0000);
	assert_eq!(u8::element_set_false_from(0b1111_1111, 8), 0b1111_1111);
}

#[test]
fn test_u8_element_is_true_from() {
	assert!(u8::element_is_true_from(0b1111_1111, 0));
	assert!(!u8::element_is_true_from(0b0000_0000, 0));
	assert!(u8::element_is_true_from(0b0111_1111, 1));
	assert!(u8::element_is_true_from(0b1111_1111, 1));
	assert!(!u8::element_is_true_from(0b1000_0000, 1));
	assert!(u8::element_is_true_from(0b1110_1111, 4));
	assert!(u8::element_is_true_from(0b0000_1111, 4));
	assert!(!u8::element_is_true_from(0b1111_0000, 4));
	assert!(u8::element_is_true_from(0b0000_0001, 7));
	assert!(!u8::element_is_true_from(0b0000_0000, 7));
	assert!(u8::element_is_true_from(0b0000_0000, 8));
}

#[test]
fn test_u8_element_set_true_from() {
	assert_eq!(u8::element_set_true_from(0b0000_0000, 0), 0b1111_1111);
	assert_eq!(u8::element_set_true_from(0b1111_1111, 0), 0b1111_1111);
	assert_eq!(u8::element_set_true_from(0b0000_0000, 4), 0b0000_1111);
	assert_eq!(u8::element_set_true_from(0b0000_1111, 4), 0b0000_1111);
	assert_eq!(u8::element_set_true_from(0b0000_1111, 8), 0b0000_1111);
	assert_eq!(u8::element_set_true_from(0b1111_0000, 8), 0b1111_0000);
	assert_eq!(u8::element_set_true_from(0b0000_0000, 8), 0b0000_0000);
}

#[test]
fn test_u8_element_contains() {
	assert!(u8::element_contains(0b0000_0000, 0, 0b0000_0000));
	assert!(u8::element_contains(0b1000_0000, 0, 0b0000_0000));
	assert!(u8::element_contains(0b0000_0000, 0, 0b1000_0000));
	assert!(u8::element_contains(0b0000_1000, 0, 0b0000_0000));
	assert!(u8::element_contains(0b0000_0000, 0, 0b1000_1000));
	assert!(u8::element_contains(0b0000_0001, 0, 0b0000_0000));
	assert!(u8::element_contains(0b0000_0000, 0, 0b1000_0001));
	assert!(u8::element_contains(0b0000_0000, 1, 0b0111_1111));
	assert!(u8::element_contains(0b1111_1111, 1, 0b1000_0000));
	assert!(u8::element_contains(0b0000_0000, 1, 0b0111_1111));
	assert!(u8::element_contains(0b1111_1111, 1, 0b1000_0000));
	assert!(u8::element_contains(0b0000_0000, 7, 0b0000_0001));
	assert!(u8::element_contains(0b1111_1111, 7, 0b1111_1110));
	assert!(!u8::element_contains(0b0000_0000, 7, 0b1000_0001));
	assert!(!u8::element_contains(0b0000_0000, 7, 0b0000_1001));
	assert!(!u8::element_contains(0b0111_1111, 7, 0b1111_1110));
	assert!(!u8::element_contains(0b1111_1111, 7, 0b1111_0110));
	assert!(u8::element_contains(0b0000_0000, 8, 0b0000_0000));
	assert!(u8::element_contains(0b1111_1111, 8, 0b1111_1111));
	assert!(!u8::element_contains(0b0000_0000, 8, 0b0000_0001));
	assert!(!u8::element_contains(0b0000_0000, 8, 0b1000_0000));
	assert!(!u8::element_contains(0b1111_1111, 8, 0b1111_1110));
	assert!(!u8::element_contains(0b1111_1111, 8, 0b0111_1111));
}

#[test]
fn test_u8_slice_inc() {
	// make sure overflow doesn't change the fixed prefix
	assert_eq!(
		u8_slice_inc([0b0000_0000, 0b0000_0000], 16),
		(true, [0b0000_0000, 0b0000_0000])
	);
	assert_eq!(
		u8_slice_inc([0b0000_0000, 0b0000_0000], 15),
		(false, [0b0000_0000, 0b0000_0001]),
	);
	assert_eq!(
		u8_slice_inc([0b0000_0000, 0b0000_0001], 15),
		(true, [0b0000_0000, 0b0000_0000]),
	);
	assert_eq!(
		u8_slice_inc([0b0000_0000, 0b0000_1011], 15),
		(true, [0b0000_0000, 0b0000_1010]),
	);
	assert_eq!(
		u8_slice_inc([0b0000_0000, 0b0000_1111], 15),
		(true, [0b0000_0000, 0b0000_1110]),
	);
	assert_eq!(
		u8_slice_inc([0b0000_0000, 0b1111_1111], 15),
		(true, [0b0000_0000, 0b1111_1110]),
	);
	assert_eq!(
		u8_slice_inc([0b0000_0001, 0b1111_1111], 15),
		(true, [0b0000_0001, 0b1111_1110]),
	);
	assert_eq!(
		u8_slice_inc([0b0000_0000, 0b0000_0000], 8),
		(false, [0b0000_0000, 0b0000_0001]),
	);
	assert_eq!(
		u8_slice_inc([0b0000_0000, 0b1111_1111], 8),
		(true, [0b0000_0000, 0b0000_0000]),
	);
	assert_eq!(
		u8_slice_inc([0b0000_0001, 0b1111_1111], 8),
		(true, [0b0000_0001, 0b0000_0000]),
	);
	assert_eq!(
		u8_slice_inc([0b0000_0000, 0b0000_0000], 0),
		(false, [0b0000_0000, 0b0000_0001]),
	);
	assert_eq!(
		u8_slice_inc([0b1111_1111, 0b1111_1111], 0),
		(true, [0b0000_0000, 0b0000_0000]),
	);
}

#[test]
fn test_u8_slice_get() {
	assert!(!u8::slice_get(&[0, 0b0000_0000], 15));
	assert!(u8::slice_get(&[0, 0b0000_0001], 15));
	assert!(!u8::slice_get(&[0, 0b0000_0000], 14));
	assert!(u8::slice_get(&[0, 0b0000_0010], 14));
	assert!(!u8::slice_get(&[0, 0b0000_0000], 8));
	assert!(u8::slice_get(&[0, 0b1000_0000], 8));
	assert!(!u8::slice_get(&[0b0000_0000, 0], 7));
	assert!(u8::slice_get(&[0b0000_0001, 0], 7));
	assert!(!u8::slice_get(&[0b0000_0000, 0], 1));
	assert!(u8::slice_get(&[0b0100_0000, 0], 1));
	assert!(!u8::slice_get(&[0b0000_0000, 0], 0));
	assert!(u8::slice_get(&[0b1000_0000, 0], 0));
}

#[test]
fn test_u8_slice_set() {
	assert_eq!(u8_slice_set([0, 0b0000_0000], 15, true), [0, 0b0000_0001]);
	assert_eq!(u8_slice_set([0, 0b0000_0001], 15, false), [0, 0b0000_0000]);
	assert_eq!(u8_slice_set([!0, 0b0000_0001], 15, true), [!0, 0b0000_0001]);
	assert_eq!(
		u8_slice_set([!0, 0b0000_0000], 15, false),
		[!0, 0b0000_0000]
	);
	assert_eq!(u8_slice_set([0, 0b0000_0000], 14, true), [0, 0b0000_0010]);
	assert_eq!(u8_slice_set([0, 0b0000_0010], 14, false), [0, 0b0000_0000]);
	assert_eq!(u8_slice_set([!0, 0b1111_1111], 14, true), [!0, 0b1111_1111]);
	assert_eq!(u8_slice_set([0, 0b0000_0000], 8, true), [0, 0b1000_0000]);
	assert_eq!(u8_slice_set([0, 0b1000_0000], 8, false), [0, 0b0000_0000]);
	assert_eq!(u8_slice_set([!0, 0b1111_1111], 8, true), [!0, 0b1111_1111]);
	assert_eq!(u8_slice_set([0b0000_0000, 0], 7, true), [0b0000_0001, 0]);
	assert_eq!(u8_slice_set([0b0000_0001, 0], 7, false), [0b0000_0000, 0]);
	assert_eq!(u8_slice_set([0b0000_0001, !0], 7, true), [0b0000_0001, !0]);
	assert_eq!(u8_slice_set([0b0000_0000, 0], 0, true), [0b1000_0000, 0]);
	assert_eq!(u8_slice_set([0b1000_0000, 0], 0, false), [0b0000_0000, 0]);
	assert_eq!(u8_slice_set([0b1111_1111, !0], 0, true), [0b1111_1111, !0]);
}

#[test]
fn test_u8_slice_flip() {
	assert_eq!(u8_slice_flip([0, 0b0000_0000], 15), [0, 0b0000_0001]);
	assert_eq!(u8_slice_flip([0, 0b0000_0001], 15), [0, 0b0000_0000]);
	assert_eq!(u8_slice_flip([0, 0b0000_0000], 8), [0, 0b1000_0000]);
	assert_eq!(u8_slice_flip([!0, 0b1111_1111], 8), [!0, 0b0111_1111]);
	assert_eq!(u8_slice_flip([0b0000_0000, 0], 7), [0b0000_0001, 0]);
	assert_eq!(u8_slice_flip([0b0000_0001, 0], 7), [0b0000_0000, 0]);
	assert_eq!(u8_slice_flip([0b0000_0000, !0], 0), [0b1000_0000, !0]);
	assert_eq!(u8_slice_flip([0b1111_1111, 0], 0), [0b0111_1111, 0]);
}

#[test]
fn test_u8_slice_shared_prefix_len() {
	assert_eq!(
		0,
		u8::slice_shared_prefix_len(&[0b0000_0000], &[0b0000_0000], 0)
	);
	assert_eq!(
		0,
		u8::slice_shared_prefix_len(&[0b0000_0000], &[0b1000_0000], 8)
	);
	assert_eq!(
		1,
		u8::slice_shared_prefix_len(&[0b0000_0000], &[0b0000_0000], 1)
	);
	assert_eq!(
		1,
		u8::slice_shared_prefix_len(&[0b0000_0000], &[0b0100_0000], 8)
	);
	assert_eq!(
		7,
		u8::slice_shared_prefix_len(&[0b1100_0000], &[0b1100_0001], 7)
	);
	assert_eq!(
		7,
		u8::slice_shared_prefix_len(&[0b1100_0000], &[0b1100_0001], 8)
	);

	assert_eq!(
		0,
		u8::slice_shared_prefix_len(&[0b0000_0000, 0b0000_0000], &[0b0000_0000, 0b0000_0000], 0)
	);
	assert_eq!(
		0,
		u8::slice_shared_prefix_len(&[0b0000_0000, 0b0000_0000], &[0b1000_0000, 0b0000_0000], 8)
	);
	assert_eq!(
		1,
		u8::slice_shared_prefix_len(&[0b0000_0000, 0b0000_0000], &[0b0000_0000, 0b0000_0000], 1)
	);
	assert_eq!(
		1,
		u8::slice_shared_prefix_len(&[0b0000_0000, 0b0000_0000], &[0b0100_0000, 0b0000_0000], 8)
	);
	assert_eq!(
		7,
		u8::slice_shared_prefix_len(&[0b1100_0000, 0b0000_0000], &[0b1100_0001, 0b0000_0000], 7)
	);
	assert_eq!(
		7,
		u8::slice_shared_prefix_len(&[0b1100_0000, 0b0000_0000], &[0b1100_0001, 0b0000_0000], 8)
	);

	assert_eq!(
		8,
		u8::slice_shared_prefix_len(&[0b0010_1000, 0b0000_0000], &[0b0010_1000, 0b0000_0000], 8)
	);
	assert_eq!(
		8,
		u8::slice_shared_prefix_len(&[0b0010_1000, 0b0000_0000], &[0b0010_1000, 0b1000_0000], 16)
	);
	assert_eq!(
		9,
		u8::slice_shared_prefix_len(&[0b0010_1000, 0b0000_0000], &[0b0010_1000, 0b0000_0000], 9)
	);
	assert_eq!(
		9,
		u8::slice_shared_prefix_len(&[0b0010_1000, 0b0000_0000], &[0b0010_1000, 0b0100_0000], 16)
	);
	assert_eq!(
		15,
		u8::slice_shared_prefix_len(&[0b0010_1000, 0b1100_0000], &[0b0010_1000, 0b1100_0001], 15)
	);
	assert_eq!(
		15,
		u8::slice_shared_prefix_len(&[0b0010_1000, 0b1100_0000], &[0b0010_1000, 0b1100_0001], 16)
	);
}

#[test]
fn test_u8_slice_set_false_from() {
	assert_eq!(
		u8_slice_set_false_from([0, 0b1111_1111], 16),
		[0, 0b1111_1111]
	);
	assert_eq!(
		u8_slice_set_false_from([!0, 0b1111_1111], 16),
		[!0, 0b1111_1111]
	);
	assert_eq!(
		u8_slice_set_false_from([!0, 0b1111_1111], 15),
		[!0, 0b1111_1110]
	);
	assert_eq!(
		u8_slice_set_false_from([!0, 0b1111_1111], 9),
		[!0, 0b1000_0000]
	);
	assert_eq!(
		u8_slice_set_false_from([0, 0b1111_1111], 8),
		[0, 0b0000_0000]
	);
	assert_eq!(
		u8_slice_set_false_from([!0, 0b1111_1111], 8),
		[!0, 0b0000_0000]
	);
	assert_eq!(
		u8_slice_set_false_from([0b1111_1111, !0], 7),
		[0b1111_1110, 0]
	);
	assert_eq!(
		u8_slice_set_false_from([0b1111_1111, !0], 1),
		[0b1000_0000, 0]
	);
	assert_eq!(
		u8_slice_set_false_from([0b1111_1111, !0], 0),
		[0b0000_0000, 0]
	);
}

#[test]
fn test_u8_slice_is_false_from() {
	assert!(u8::slice_is_false_from(&[0, 0b1111_1111], 16));
	assert!(u8::slice_is_false_from(&[!0, 0b1111_1111], 16));
	assert!(u8::slice_is_false_from(&[!0, 0b1111_1110], 15));
	assert!(u8::slice_is_false_from(&[!0, 0b1000_0000], 9));
	assert!(u8::slice_is_false_from(&[0, 0b0000_0000], 8));
	assert!(u8::slice_is_false_from(&[!0, 0b0000_0000], 8));
	assert!(u8::slice_is_false_from(&[0b1111_1110, 0], 7));
	assert!(u8::slice_is_false_from(&[0b1000_0000, 0], 1));
	assert!(u8::slice_is_false_from(&[0b0000_0000, 0], 0));
}

#[test]
fn test_u8_slice_set_true_from() {
	assert_eq!(
		u8_slice_set_true_from([!0, 0b1111_1111], 16),
		[!0, 0b1111_1111]
	);
	assert_eq!(
		u8_slice_set_true_from([0, 0b0000_0000], 16),
		[0, 0b0000_0000]
	);
	assert_eq!(
		u8_slice_set_true_from([0, 0b0000_0000], 15),
		[0, 0b0000_0001]
	);
	assert_eq!(
		u8_slice_set_true_from([0, 0b0000_0000], 9),
		[0, 0b0111_1111]
	);
	assert_eq!(
		u8_slice_set_true_from([!0, 0b0000_0000], 8),
		[!0, 0b1111_1111]
	);
	assert_eq!(
		u8_slice_set_true_from([0, 0b0000_0000], 8),
		[0, 0b1111_1111]
	);
	assert_eq!(
		u8_slice_set_true_from([0b0000_0000, 0], 7),
		[0b0000_0001, !0]
	);
	assert_eq!(
		u8_slice_set_true_from([0b0000_0000, 0], 1),
		[0b0111_1111, !0]
	);
	assert_eq!(
		u8_slice_set_true_from([0b0000_0000, 0], 0),
		[0b1111_1111, !0]
	);
}

#[test]
fn test_u8_slice_is_true_from() {
	assert!(u8::slice_is_true_from(&[!0, 0b1111_1111], 16));
	assert!(u8::slice_is_true_from(&[0, 0b0000_0000], 16));
	assert!(u8::slice_is_true_from(&[0, 0b0000_0001], 15));
	assert!(u8::slice_is_true_from(&[0, 0b0111_1111], 9));
	assert!(u8::slice_is_true_from(&[!0, 0b1111_1111], 8));
	assert!(u8::slice_is_true_from(&[0, 0b1111_1111], 8));
	assert!(u8::slice_is_true_from(&[0b0000_0001, !0], 7));
	assert!(u8::slice_is_true_from(&[0b0111_1111, !0], 1));
	assert!(u8::slice_is_true_from(&[0b1111_1111, !0], 0));
}

#[test]
fn test_u8_slice_contains() {
	assert!(u8::slice_contains(&[0, 0], 0, &[!0, !0]));
	assert!(u8::slice_contains(&[0b0000_0000, 0], 1, &[0b0111_1111, !0]));
	assert!(u8::slice_contains(&[0b1111_1111, !0], 1, &[0b1000_0000, 0]));
	assert!(u8::slice_contains(&[0b0000_0000, 0], 7, &[0b0000_0001, !0]));
	assert!(u8::slice_contains(&[0b1111_1111, 0], 7, &[0b1111_1110, !0]));
	assert!(!u8::slice_contains(&[0b0000_0000, 0], 7, &[0b1000_0001, 0]));
	assert!(!u8::slice_contains(&[0b0000_0000, 0], 7, &[0b0000_1001, 0]));
	assert!(!u8::slice_contains(&[0b0111_1111, 0], 7, &[0b1111_1110, 0]));
	assert!(!u8::slice_contains(&[0b1111_1111, 0], 7, &[0b1111_0110, 0]));
	assert!(u8::slice_contains(&[0b0000_0000, 0], 8, &[0b0000_0000, !0]));
	assert!(u8::slice_contains(&[0b1111_1111, !0], 8, &[0b1111_1111, 0]));
	assert!(!u8::slice_contains(&[0b0000_0000, 0], 8, &[0b0000_0001, 0]));
	assert!(!u8::slice_contains(&[0b0000_0000, 0], 8, &[0b1000_0000, 0]));
	assert!(!u8::slice_contains(&[0b1111_1111, 0], 8, &[0b1111_1110, 0]));
	assert!(!u8::slice_contains(&[0b1111_1111, 0], 8, &[0b0111_1111, 0]));

	assert!(u8::slice_contains(&[0, 0b0000_0000], 9, &[0, 0b0111_1111]));
	assert!(u8::slice_contains(
		&[!0, 0b1111_1111],
		9,
		&[!0, 0b1000_0000]
	));
	assert!(u8::slice_contains(&[0, 0b0000_0000], 15, &[0, 0b0000_0001]));
	assert!(u8::slice_contains(
		&[!0, 0b1111_1111],
		15,
		&[!0, 0b1111_1110]
	));
	assert!(!u8::slice_contains(
		&[0, 0b0000_0000],
		15,
		&[0, 0b1000_0001]
	));
	assert!(!u8::slice_contains(
		&[0, 0b0000_0000],
		15,
		&[0, 0b0000_1001]
	));
	assert!(!u8::slice_contains(
		&[!0, 0b0111_1111],
		15,
		&[!0, 0b1111_1110]
	));
	assert!(!u8::slice_contains(
		&[!0, 0b1111_1111],
		15,
		&[!0, 0b1111_0110]
	));
	assert!(u8::slice_contains(&[0, 0b0000_0000], 16, &[0, 0b0000_0000]));
	assert!(u8::slice_contains(
		&[!0, 0b1111_1111],
		16,
		&[!0, 0b1111_1111]
	));
	assert!(!u8::slice_contains(
		&[0, 0b0000_0000],
		16,
		&[0, 0b0000_0001]
	));
	assert!(!u8::slice_contains(
		&[0, 0b0000_0000],
		16,
		&[0, 0b1000_0000]
	));
	assert!(!u8::slice_contains(
		&[!0, 0b1111_1111],
		16,
		&[!0, 0b1111_1110]
	));
	assert!(!u8::slice_contains(
		&[!0, 0b1111_1111],
		16,
		&[!0, 0b0111_1111]
	));
}
