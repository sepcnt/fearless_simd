// Copyright 2026 the Fearless_SIMD Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use fearless_simd::*;
use fearless_simd_dev_macros::simd_test;

// Stable scalar order, independent of the permutation tables and their cursors.
fn expected<T: Copy, const N: usize>(lanes: [T; N], bits: u64) -> ([T; N], usize) {
    let mut order = core::array::from_fn::<_, N, _>(|i| i);
    order.sort_by_key(|i| bits & (1 << i) == 0);
    let count = order.partition_point(|i| bits & (1 << i) != 0);
    (order.map(|i| lanes[i]), count)
}

#[simd_test]
fn partition_dwords<S: Simd>(s: S) {
    let words = [0x8000_0000, 0, 0x7fc0_0123, 0xff80_0001, 1, 3, 5, 7];
    let values = f32x8::simd_from(s, words.map(f32::from_bits));
    for bits in 0..256 {
        let (v, n) = values.partition(mask32x8::from_bitmask(s, bits));
        assert_eq!((v.to_array().map(f32::to_bits), n), expected(words, bits));
    }
}

#[simd_test]
fn partition_qwords<S: Simd>(s: S) {
    let words = [
        0x8000_0000_0000_0000,
        0,
        0x7ff8_abcd_0123_4567,
        0xfff0_1234_89ab_cdef,
    ];
    let values = f64x4::simd_from(s, words.map(f64::from_bits));
    for bits in 0..16 {
        let (v, n) = values.partition(mask64x4::from_bitmask(s, bits));
        assert_eq!((v.to_array().map(f64::to_bits), n), expected(words, bits));
    }
}

#[simd_test]
fn partition_wide_bytes<S: Simd>(s: S) {
    let bytes = core::array::from_fn(|i| i as u8);
    let v = u8x64::simd_from(s, bytes);
    for bits in [
        0,
        u64::MAX,
        1 << 63,
        0xffff_ffff_0000_0000,
        0x5555_aaaa_3333_cccc,
    ] {
        let (out, n) = v.partition(mask8x64::from_bitmask(s, bits));
        assert_eq!((out.to_array(), n), expected(bytes, bits));
    }
}
