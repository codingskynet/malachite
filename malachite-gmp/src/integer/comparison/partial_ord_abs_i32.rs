use integer::Integer;
use std::cmp::Ordering;
use malachite_base::traits::PartialOrdAbs;

/// Compares the absolute value of an `Integer` to the absolute value of an `i32`.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::PartialOrdAbs;
/// use malachite_gmp::integer::Integer;
///
/// fn main() {
///     assert!(Integer::from(-123).gt_abs(&-122));
///     assert!(Integer::from(-123).ge_abs(&-122));
///     assert!(Integer::from(-123).lt_abs(&-124));
///     assert!(Integer::from(-123).le_abs(&-124));
///     assert!(Integer::trillion().gt_abs(&123));
///     assert!(Integer::trillion().ge_abs(&123));
///     assert!((-Integer::trillion()).gt_abs(&123));
///     assert!((-Integer::trillion()).ge_abs(&123));
/// }
/// ```
impl PartialOrdAbs<i32> for Integer {
    fn partial_cmp_abs(&self, other: &i32) -> Option<Ordering> {
        self.partial_cmp_abs(&(other.wrapping_abs() as u32))
    }
}

/// Compares the absolute value of an `i32` to the absolute value of an `Integer`.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::PartialOrdAbs;
/// use malachite_gmp::integer::Integer;
///
/// fn main() {
///     assert!((-122).lt_abs(&Integer::from(-123)));
///     assert!((-122).le_abs(&Integer::from(-123)));
///     assert!((-124).gt_abs(&Integer::from(-123)));
///     assert!((-123).ge_abs(&Integer::from(-123)));
///     assert!(123.lt_abs(&Integer::trillion()));
///     assert!(123.le_abs(&Integer::trillion()));
///     assert!(123.lt_abs(&(-Integer::trillion())));
///     assert!(123.le_abs(&(-Integer::trillion())));
/// }
/// ```
impl PartialOrdAbs<Integer> for i32 {
    fn partial_cmp_abs(&self, other: &Integer) -> Option<Ordering> {
        (self.wrapping_abs() as u32).partial_cmp_abs(other)
    }
}
