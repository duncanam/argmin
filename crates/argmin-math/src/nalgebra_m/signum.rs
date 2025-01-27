// Copyright 2018-2024 argmin developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::{Allocator, ArgminSignum};

use nalgebra::{base::dimension::Dim, DefaultAllocator, OMatrix, SimdComplexField};

impl<N, R, C> ArgminSignum for OMatrix<N, R, C>
where
    N: SimdComplexField,
    R: Dim,
    C: Dim,
    DefaultAllocator: Allocator<N, R, C>,
{
    #[inline]
    fn signum(self) -> OMatrix<N, R, C> {
        self.map(|v| v.simd_signum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use nalgebra::{Matrix2x3, Vector3};
    use paste::item;

    macro_rules! make_test {
        ($t:ty) => {
            item! {
                #[test]
                fn [<test_signum_ $t>]() {
                    let a = Vector3::new(3 as $t, -4 as $t, -8 as $t);
                    let b = Vector3::new(1 as $t, -1 as $t, -1 as $t);
                    let res = <Vector3<$t> as ArgminSignum>::signum(a);
                    for i in 0..3 {
                        assert_relative_eq!(b[i], res[i], epsilon = $t::EPSILON);
                    }
                }
            }

            item! {
                #[test]
                fn [<test_signum_scalar_mat_2_ $t>]() {
                    let b = Matrix2x3::new(
                        3 as $t, -4 as $t, 8 as $t,
                        -2 as $t, -5 as $t, 9 as $t
                    );
                    let target = Matrix2x3::new(
                        1 as $t, -1 as $t, 1 as $t,
                        -1 as $t, -1 as $t, 1 as $t
                    );
                    let res = b.signum();
                    for i in 0..3 {
                        for j in 0..2 {
                            assert_relative_eq!(target[(j, i)], res[(j, i)], epsilon = $t::EPSILON);
                        }
                    }
                }
            }
        };
    }

    make_test!(f32);
    make_test!(f64);
}
