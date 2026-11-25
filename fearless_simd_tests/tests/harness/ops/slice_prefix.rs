// Copyright 2026 the Fearless_SIMD Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use fearless_simd::*;
use fearless_simd_dev_macros::simd_test;

#[simd_test]
fn slice_prefix<S: Simd>(s: S) {
    macro_rules! lengths {
        ($vec:ident, $bits:ty, $float:ty, $n:literal, $nan:expr) => {{
            let negative_zero: $float = -0.0;
            let guard: $float = 1.0;
            let bits: [$bits; $n + 3] = core::array::from_fn(|i| match i % 4 {
                0 => negative_zero.to_bits(),
                1 => 0,
                _ => $nan + i as $bits,
            });
            let src = bits.map(<$float>::from_bits);
            let fill = <$float>::from_bits($nan);
            for len in 0..=$n + 2 {
                let v = $vec::load_prefix_or(s, &src[1..1 + len], fill);
                let expected = core::array::from_fn(|i| if i < len { bits[1 + i] } else { $nan });
                assert_eq!(v.to_array().map(<$float>::to_bits), expected);
                // Distinguish writes to the supplied prefix from untouched guards/excess.
                let mut dst = [guard; $n + 4];
                v.store_prefix(&mut dst[1..1 + len]);
                let count = len.min($n);
                let mut expected = [guard.to_bits(); $n + 4];
                expected[1..1 + count].copy_from_slice(&bits[1..1 + count]);
                assert_eq!(dst.map(<$float>::to_bits), expected);
            }
        }};
    }
    lengths!(f32x8, u32, f32, 8, 0x7fc0_0001);
    lengths!(f64x2, u64, f64, 2, 0xfff0_0000_0000_0001);
    lengths!(f64x8, u64, f64, 8, 0x7ff8_0000_0000_0001);
}
