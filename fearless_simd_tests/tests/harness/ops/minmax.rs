// Copyright 2026 the Fearless_SIMD Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use fearless_simd::*;
use fearless_simd_dev_macros::simd_test;

#[simd_test]
fn minmax<S: Simd>(s: S) {
    macro_rules! lanes {
        ($vec:ident, $int:ty) => {{
            // Include equal lanes, both comparison directions, and the unsigned sign boundary.
            let a = $vec::from_fn(s, |i| [<$int>::MAX, <$int>::MIN, 1, 1][i % 4]);
            let b = $vec::from_fn(s, |i| [<$int>::MIN, <$int>::MAX, 1, 0][i % 4]);
            let (lo, hi) = a.minmax(b);
            for i in 0..a.len() {
                assert_eq!((lo[i], hi[i]), (a[i].min(b[i]), a[i].max(b[i])));
            }
        }};
    }
    lanes!(i8x16, i8);
    lanes!(u8x16, u8);
    lanes!(i16x8, i16);
    lanes!(u16x8, u16);
    lanes!(i32x8, i32);
    lanes!(u32x8, u32);
    lanes!(i64x2, i64);
    lanes!(i64x4, i64);
    lanes!(u64x4, u64);
    lanes!(u64x8, u64);
}
