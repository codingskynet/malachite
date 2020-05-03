use std::ops::{Shl, ShlAssign};

use malachite_base::num::arithmetic::traits::UnsignedAbs;

use integer::Integer;

macro_rules! impl_integer_shl_signed {
    ($t:ident) => {
        impl Shl<$t> for Integer {
            type Output = Integer;

            /// Shifts an `Integer` left (multiplies it by a power of 2 or divides it by a power of
            /// 2 and takes the floor), taking the `Integer` by value.
            ///
            /// Time: worst case O(`other`)
            ///
            /// Additional memory: worst case O(`other`)
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use malachite_base::num::basic::traits::Zero;
            /// use malachite_nz::integer::Integer;
            ///
            /// assert_eq!((Integer::ZERO << 10i8).to_string(), "0");
            /// assert_eq!((Integer::from(123) << 2i16).to_string(), "492");
            /// assert_eq!((Integer::from(123) << 100i32).to_string(),
            ///     "155921023828072216384094494261248");
            /// assert_eq!((Integer::from(-123) << 2i64).to_string(), "-492");
            /// assert_eq!((Integer::from(-123) << 100i8).to_string(),
            ///     "-155921023828072216384094494261248");
            /// assert_eq!((Integer::ZERO << -10i16).to_string(), "0");
            /// assert_eq!((Integer::from(492) << -2i32).to_string(), "123");
            /// assert_eq!((-Integer::trillion() << -10i64).to_string(), "-976562500");
            /// ```
            #[inline]
            fn shl(mut self, other: $t) -> Integer {
                self <<= other;
                self
            }
        }

        impl<'a> Shl<$t> for &'a Integer {
            type Output = Integer;

            /// Shifts an `Integer` left (multiplies it by a power of 2 or divides it by a power of
            /// 2 and takes the floor), taking the `Integer` by reference.
            ///
            /// Time: worst case O(`other`)
            ///
            /// Additional memory: worst case O(`other`)
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use malachite_base::num::basic::traits::Zero;
            /// use malachite_nz::integer::Integer;
            ///
            /// assert_eq!((&Integer::ZERO << 10i8).to_string(), "0");
            /// assert_eq!((&Integer::from(123) << 2i16).to_string(), "492");
            /// assert_eq!((&Integer::from(123) << 100i32).to_string(),
            ///     "155921023828072216384094494261248");
            /// assert_eq!((&Integer::from(-123) << 2i64).to_string(), "-492");
            /// assert_eq!((&Integer::from(-123) << 100i8).to_string(),
            ///     "-155921023828072216384094494261248");
            /// assert_eq!((&Integer::ZERO << -10i16).to_string(), "0");
            /// assert_eq!((&Integer::from(492) << -2i32).to_string(), "123");
            /// assert_eq!((&(-Integer::trillion()) << -10i64).to_string(), "-976562500");
            /// ```
            fn shl(self, other: $t) -> Integer {
                if other >= 0 {
                    self << other.unsigned_abs()
                } else {
                    self >> other.unsigned_abs()
                }
            }
        }

        impl ShlAssign<$t> for Integer {
            /// Shifts an `Integer` left (multiplies it by a power of 2 or divides it by a power of
            /// 2 and takes the floor) in place.
            ///
            /// Time: worst case O(`other`)
            ///
            /// Additional memory: worst case O(`other`)
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use malachite_base::num::basic::traits::{NegativeOne, One};
            /// use malachite_nz::integer::Integer;
            ///
            /// let mut x = Integer::ONE;
            /// x <<= 1i8;
            /// x <<= 2i16;
            /// x <<= 3i32;
            /// x <<= 4i64;
            /// assert_eq!(x.to_string(), "1024");
            /// let mut x = Integer::NEGATIVE_ONE;
            /// x <<= 1i8;
            /// x <<= 2i16;
            /// x <<= 3i32;
            /// x <<= 4i64;
            /// assert_eq!(x.to_string(), "-1024");
            ///
            /// let mut x = Integer::from(1024);
            /// x <<= -1i8;
            /// x <<= -2i16;
            /// x <<= -3i32;
            /// x <<= -4i64;
            /// assert_eq!(x.to_string(), "1");
            /// ```
            fn shl_assign(&mut self, other: $t) {
                if other >= 0 {
                    *self <<= other.unsigned_abs();
                } else {
                    *self >>= other.unsigned_abs();
                }
            }
        }
    };
}
impl_integer_shl_signed!(i8);
impl_integer_shl_signed!(i16);
impl_integer_shl_signed!(i32);
impl_integer_shl_signed!(i64);
impl_integer_shl_signed!(i128);
impl_integer_shl_signed!(isize);
