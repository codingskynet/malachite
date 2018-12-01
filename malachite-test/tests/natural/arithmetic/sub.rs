use common::test_properties;
use malachite_base::misc::{CheckedFrom, Max};
use malachite_base::num::{One, PrimitiveInteger, Zero};
use malachite_nz::integer::Integer;
use malachite_nz::natural::arithmetic::sub::{
    limbs_sub, limbs_sub_in_place_left, limbs_sub_in_place_right,
    limbs_sub_same_length_in_place_left, limbs_sub_same_length_in_place_right,
    limbs_sub_same_length_to_out, limbs_sub_to_out,
};
use malachite_nz::natural::Natural;
use malachite_test::common::{
    biguint_to_natural, natural_to_biguint, natural_to_rug_integer, rug_integer_to_natural,
};
use malachite_test::inputs::base::pairs_of_unsigneds_var_1;
use malachite_test::inputs::base::{
    pairs_of_unsigned_vec_var_1, pairs_of_unsigned_vec_var_3, triples_of_unsigned_vec_var_3,
    triples_of_unsigned_vec_var_9,
};
use malachite_test::inputs::natural::{
    naturals, pairs_of_natural_and_u32_var_1, pairs_of_naturals_var_1,
    pairs_of_u32_and_natural_var_1,
};
use num::BigUint;
use rug;
use std::str::FromStr;

#[test]
fn test_limbs_sub() {
    let test = |xs, ys, out| {
        assert_eq!(limbs_sub(xs, ys), out);
    };
    test(&[], &[], (vec![], false));
    test(&[2], &[], (vec![2], false));
    test(&[3], &[2], (vec![1], false));
    test(&[2], &[3], (vec![4_294_967_295], true));
    test(&[1, 2, 3], &[1, 1, 1], (vec![0, 1, 2], false));
    test(
        &[1, 1, 1],
        &[1, 2, 3],
        (vec![0, 4_294_967_295, 4_294_967_293], true),
    );
    test(
        &[1, 2, 3],
        &[6, 7],
        (vec![4_294_967_291, 4_294_967_290, 2], false),
    );
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        (vec![4_294_967_294, 4_294_967_295, 1], false),
    );
    test(
        &[102, 101, 100],
        &[100, 101, 102],
        (vec![2, 0, 4_294_967_294], true),
    );
    test(
        &[0, 0, 0],
        &[1],
        (vec![4_294_967_295, 4_294_967_295, 4_294_967_295], true),
    );
}

#[test]
#[should_panic(expected = "assertion failed: xs_len >= ys_len")]
fn limbs_sub_fail() {
    limbs_sub(&[6, 7], &[1, 2, 3]);
}

#[test]
fn test_limbs_sub_same_length_to_out() {
    let test = |xs, ys, out_before: &[u32], borrow, out_after| {
        let mut out = out_before.to_vec();
        assert_eq!(limbs_sub_same_length_to_out(&mut out, xs, ys), borrow);
        assert_eq!(out, out_after);
    };
    test(&[], &[], &[0, 0], false, vec![0, 0]);
    test(&[3], &[2], &[0, 0], false, vec![1, 0]);
    test(&[2], &[3], &[0, 0], true, vec![4_294_967_295, 0]);
    test(
        &[1, 2, 3],
        &[1, 1, 1],
        &[0, 1, 2, 5],
        false,
        vec![0, 1, 2, 5],
    );
    test(
        &[1, 1, 1],
        &[1, 2, 3],
        &[5, 5, 5, 5],
        true,
        vec![0, 4_294_967_295, 4_294_967_293, 5],
    );
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        &[10, 10, 10, 10],
        false,
        vec![4_294_967_294, 4_294_967_295, 1, 10],
    );
    test(
        &[102, 101, 100],
        &[100, 101, 102],
        &[10, 10, 10, 10],
        true,
        vec![2, 0, 4_294_967_294, 10],
    );
    test(
        &[0, 0, 0],
        &[1, 0, 0],
        &[10, 10, 10, 10],
        true,
        vec![4_294_967_295, 4_294_967_295, 4_294_967_295, 10],
    );
}

#[test]
#[should_panic(expected = "assertion failed: out_limbs.len() >= len")]
fn limbs_sub_same_length_to_out_fail_1() {
    let mut out = vec![10, 10];
    limbs_sub_same_length_to_out(&mut out, &[6, 7, 8], &[1, 2, 3]);
}

#[test]
#[should_panic(expected = "assertion failed: `(left == right)`")]
fn limbs_sub_same_length_to_out_fail_2() {
    let mut out = vec![10, 10, 10];
    limbs_sub_same_length_to_out(&mut out, &[6, 7, 8], &[1, 2]);
}

#[test]
fn test_limbs_sub_to_out() {
    let test = |xs, ys, out_before: &[u32], borrow, out_after| {
        let mut out = out_before.to_vec();
        assert_eq!(limbs_sub_to_out(&mut out, xs, ys), borrow);
        assert_eq!(out, out_after);
    };
    test(&[], &[], &[0, 0], false, vec![0, 0]);
    test(&[2], &[], &[0, 0], false, vec![2, 0]);
    test(&[3], &[2], &[0, 0], false, vec![1, 0]);
    test(&[2], &[3], &[0, 0], true, vec![4_294_967_295, 0]);
    test(
        &[1, 2, 3],
        &[1, 1, 1],
        &[0, 1, 2, 5],
        false,
        vec![0, 1, 2, 5],
    );
    test(
        &[1, 1, 1],
        &[1, 2, 3],
        &[5, 5, 5, 5],
        true,
        vec![0, 4_294_967_295, 4_294_967_293, 5],
    );
    test(
        &[1, 2, 3],
        &[6, 7],
        &[0, 0, 0],
        false,
        vec![4_294_967_291, 4_294_967_290, 2],
    );
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        &[10, 10, 10, 10],
        false,
        vec![4_294_967_294, 4_294_967_295, 1, 10],
    );
    test(
        &[102, 101, 100],
        &[100, 101, 102],
        &[10, 10, 10, 10],
        true,
        vec![2, 0, 4_294_967_294, 10],
    );
    test(
        &[0, 0, 0],
        &[1],
        &[10, 10, 10, 10],
        true,
        vec![4_294_967_295, 4_294_967_295, 4_294_967_295, 10],
    );
}

#[test]
#[should_panic(expected = "assertion failed: out_limbs.len() >= xs_len")]
fn limbs_sub_to_out_fail_1() {
    let mut out = vec![10, 10];
    limbs_sub_to_out(&mut out, &[6, 7, 8], &[1, 2]);
}

#[test]
#[should_panic(expected = "assertion failed: xs_len >= ys_len")]
fn limbs_sub_to_out_fail_2() {
    let mut out = vec![10, 10, 10];
    limbs_sub_to_out(&mut out, &[6, 7], &[1, 2, 3]);
}

#[test]
fn test_limbs_sub_same_length_in_place_left() {
    let test = |xs_before: &[u32], ys, borrow, xs_after| {
        let mut xs = xs_before.to_vec();
        assert_eq!(limbs_sub_same_length_in_place_left(&mut xs, ys), borrow);
        assert_eq!(xs, xs_after);
    };
    test(&[], &[], false, vec![]);
    test(&[3], &[2], false, vec![1]);
    test(&[2], &[3], true, vec![4_294_967_295]);
    test(&[1, 2, 3], &[1, 1, 1], false, vec![0, 1, 2]);
    test(
        &[1, 1, 1],
        &[1, 2, 3],
        true,
        vec![0, 4_294_967_295, 4_294_967_293],
    );
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        false,
        vec![4_294_967_294, 4_294_967_295, 1],
    );
    test(
        &[102, 101, 100],
        &[100, 101, 102],
        true,
        vec![2, 0, 4_294_967_294],
    );
    test(
        &[0, 0, 0],
        &[1, 0, 0],
        true,
        vec![4_294_967_295, 4_294_967_295, 4_294_967_295],
    );
}

#[test]
#[should_panic(expected = "assertion failed: `(left == right)")]
fn limbs_sub_same_length_in_place_left_fail_1() {
    limbs_sub_same_length_in_place_left(&mut [6, 7], &[1, 2, 3]);
}

#[test]
#[should_panic(expected = "assertion failed: `(left == right)")]
fn limbs_sub_same_length_in_place_left_fail_2() {
    limbs_sub_same_length_in_place_left(&mut [1, 2], &[6, 7, 8]);
}

#[test]
fn test_limbs_sub_in_place_left() {
    let test = |xs_before: &[u32], ys, borrow, xs_after| {
        let mut xs = xs_before.to_vec();
        assert_eq!(limbs_sub_in_place_left(&mut xs, ys), borrow);
        assert_eq!(xs, xs_after);
    };
    test(&[], &[], false, vec![]);
    test(&[2], &[], false, vec![2]);
    test(&[3], &[2], false, vec![1]);
    test(&[2], &[3], true, vec![4_294_967_295]);
    test(&[1, 2, 3], &[1, 1, 1], false, vec![0, 1, 2]);
    test(
        &[1, 1, 1],
        &[1, 2, 3],
        true,
        vec![0, 4_294_967_295, 4_294_967_293],
    );
    test(
        &[1, 2, 3],
        &[6, 7],
        false,
        vec![4_294_967_291, 4_294_967_290, 2],
    );
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        false,
        vec![4_294_967_294, 4_294_967_295, 1],
    );
    test(
        &[102, 101, 100],
        &[100, 101, 102],
        true,
        vec![2, 0, 4_294_967_294],
    );
    test(
        &[0, 0, 0],
        &[1],
        true,
        vec![4_294_967_295, 4_294_967_295, 4_294_967_295],
    );
}

#[test]
#[should_panic(expected = "assertion failed: xs_len >= ys_len")]
fn limbs_sub_in_place_left_fail() {
    limbs_sub_in_place_left(&mut [6, 7], &[1, 2, 3]);
}

#[test]
fn test_limbs_sub_same_length_in_place_right() {
    let test = |xs, ys_before: &[u32], borrow, ys_after| {
        let mut ys = ys_before.to_vec();
        assert_eq!(limbs_sub_same_length_in_place_right(xs, &mut ys), borrow);
        assert_eq!(ys, ys_after);
    };
    test(&[], &[], false, vec![]);
    test(&[3], &[2], false, vec![1]);
    test(&[2], &[3], true, vec![4_294_967_295]);
    test(&[1, 2, 3], &[1, 1, 1], false, vec![0, 1, 2]);
    test(
        &[1, 1, 1],
        &[1, 2, 3],
        true,
        vec![0, 4_294_967_295, 4_294_967_293],
    );
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        false,
        vec![4_294_967_294, 4_294_967_295, 1],
    );
    test(
        &[102, 101, 100],
        &[100, 101, 102],
        true,
        vec![2, 0, 4_294_967_294],
    );
    test(
        &[0, 0, 0],
        &[1, 0, 0],
        true,
        vec![4_294_967_295, 4_294_967_295, 4_294_967_295],
    );
}

#[test]
#[should_panic(expected = "assertion failed: `(left == right)")]
fn limbs_sub_same_length_in_place_right_fail_1() {
    limbs_sub_same_length_in_place_right(&[6, 7], &mut vec![1, 2, 3]);
}

#[test]
#[should_panic(expected = "assertion failed: `(left == right)")]
fn limbs_sub_same_length_in_place_right_fail_2() {
    limbs_sub_same_length_in_place_right(&[1, 2], &mut vec![6, 7, 8]);
}

#[test]
fn test_limbs_sub_in_place_right() {
    let test = |xs, ys_before: &[u32], borrow, ys_after| {
        let mut ys = ys_before.to_vec();
        assert_eq!(limbs_sub_in_place_right(xs, &mut ys), borrow);
        assert_eq!(ys, ys_after);
    };
    test(&[], &[], false, vec![]);
    test(&[2], &[], false, vec![2]);
    test(&[3], &[2], false, vec![1]);
    test(&[2], &[3], true, vec![4_294_967_295]);
    test(&[1, 2, 3], &[1, 1, 1], false, vec![0, 1, 2]);
    test(
        &[1, 1, 1],
        &[1, 2, 3],
        true,
        vec![0, 4_294_967_295, 4_294_967_293],
    );
    test(
        &[1, 2, 3],
        &[6, 7],
        false,
        vec![4_294_967_291, 4_294_967_290, 2],
    );
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        false,
        vec![4_294_967_294, 4_294_967_295, 1],
    );
    test(
        &[102, 101, 100],
        &[100, 101, 102],
        true,
        vec![2, 0, 4_294_967_294],
    );
    test(
        &[0, 0, 0],
        &[1],
        true,
        vec![4_294_967_295, 4_294_967_295, 4_294_967_295],
    );
}

#[test]
#[should_panic(expected = "assertion failed: xs_len >= ys_len")]
fn limbs_sub_in_place_right_fail() {
    limbs_sub_in_place_right(&[6, 7], &mut vec![1, 2, 3]);
}

#[test]
fn test_sub_natural() {
    let test = |u, v, out| {
        let mut n = Natural::from_str(u).unwrap();
        n -= Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = Natural::from_str(u).unwrap();
        n -= &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u).unwrap() - Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u).unwrap() - &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Natural::from_str(u).unwrap() - Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Natural::from_str(u).unwrap() - &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = BigUint::from_str(u).unwrap() - BigUint::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);

        let n = rug::Integer::from_str(u).unwrap() - rug::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
    };
    test("0", "0", "0");
    test("123", "0", "123");
    test("456", "123", "333");
    test("1000000000000", "123", "999999999877");
    test("12345678987654321", "314159265358979", "12031519722295342");
    test("4294967296", "1", "4294967295");
    test("4294967295", "4294967295", "0");
    test("4294967296", "4294967295", "1");
    test("4294967296", "4294967296", "0");
    test("18446744073709551616", "1", "18446744073709551615");
    test("18446744073709551615", "18446744073709551615", "0");
    test("18446744073709551616", "18446744073709551615", "1");
    test("70734740290631708", "282942734368", "70734457347897340");
}

#[test]
#[should_panic(expected = "Cannot subtract a Natural from a smaller Natural")]
fn sub_assign_fail() {
    let mut x = Natural::from_str("123").unwrap();
    x -= Natural::from_str("456").unwrap();
}

#[test]
#[should_panic(expected = "Cannot subtract a Natural from a smaller Natural")]
fn sub_assign_ref_fail() {
    let mut x = Natural::from_str("123").unwrap();
    x -= &Natural::from_str("456").unwrap();
}

#[test]
#[should_panic(expected = "Cannot subtract a Natural from a smaller Natural")]
#[allow(unused_must_use)]
fn sub_fail_1() {
    Natural::from(123_u32) - Natural::from(456_u32);
}

#[test]
#[should_panic(expected = "Cannot subtract a Natural from a smaller Natural")]
#[allow(unused_must_use)]
fn sub_fail_2() {
    Natural::from(123_u32) - &Natural::from(456_u32);
}

#[test]
#[should_panic(expected = "Cannot subtract a Natural from a smaller Natural")]
#[allow(unused_must_use)]
fn sub_fail_3() {
    &Natural::from(123_u32) - Natural::from(456_u32);
}

#[test]
#[should_panic(
    expected = "Cannot subtract a Natural from a smaller Natural. self: 123, other: 456"
)]
#[allow(unused_must_use)]
fn sub_fail_4() {
    &Natural::from(123_u32) - &Natural::from(456_u32);
}

#[test]
fn limbs_sub_properties() {
    test_properties(pairs_of_unsigned_vec_var_3, |&(ref xs, ref ys)| {
        let (limbs, borrow) = limbs_sub(xs, ys);
        let len = limbs.len() as u32;
        let n = Natural::from_owned_limbs_asc(limbs);
        if borrow {
            assert_eq!(
                n,
                Integer::from(Natural::from_limbs_asc(xs))
                    - Integer::from(Natural::from_limbs_asc(ys))
                    + (Integer::ONE << (u32::WIDTH * len))
            );
        } else {
            assert_eq!(n, Natural::from_limbs_asc(xs) - Natural::from_limbs_asc(ys));
        }
    });
}

fn limbs_sub_to_out_helper(
    f: &mut FnMut(&mut [u32], &[u32], &[u32]) -> bool,
    out_limbs: &Vec<u32>,
    xs: &Vec<u32>,
    ys: &Vec<u32>,
) {
    let mut out_limbs = out_limbs.to_vec();
    let old_out_limbs = out_limbs.clone();
    let len = xs.len();
    let limbs = if f(&mut out_limbs, xs, ys) {
        let n = Integer::from(Natural::from_limbs_asc(xs))
            - Integer::from(Natural::from_limbs_asc(ys))
            + (Integer::ONE << (u32::WIDTH * (len as u32)));
        let mut limbs = Natural::checked_from(n).unwrap().into_limbs_asc();
        limbs.resize(len, u32::MAX);
        limbs
    } else {
        let n = Natural::from_limbs_asc(xs) - Natural::from_limbs_asc(ys);
        let mut limbs = n.into_limbs_asc();
        limbs.resize(len, 0);
        limbs
    };
    assert_eq!(limbs, &out_limbs[..len]);
    assert_eq!(&out_limbs[len..], &old_out_limbs[len..]);
}

#[test]
fn limbs_sub_same_length_to_out_properties() {
    test_properties(
        triples_of_unsigned_vec_var_3,
        |&(ref out_limbs, ref xs, ref ys)| {
            limbs_sub_to_out_helper(&mut limbs_sub_same_length_to_out, out_limbs, xs, ys);
        },
    );
}

#[test]
fn limbs_sub_to_out_properties() {
    test_properties(
        triples_of_unsigned_vec_var_9,
        |&(ref out_limbs, ref xs, ref ys)| {
            limbs_sub_to_out_helper(&mut limbs_sub_to_out, out_limbs, xs, ys);
        },
    );
}

fn limbs_sub_in_place_left_helper(
    f: &mut FnMut(&mut [u32], &[u32]) -> bool,
    xs: &Vec<u32>,
    ys: &Vec<u32>,
) {
    let mut xs = xs.to_vec();
    let xs_old = xs.clone();
    let len = xs.len() as u32;
    let borrow = f(&mut xs, ys);
    let n = Natural::from_owned_limbs_asc(xs);
    if borrow {
        assert_eq!(
            n,
            Integer::from(Natural::from_owned_limbs_asc(xs_old))
                - Integer::from(Natural::from_limbs_asc(ys))
                + (Integer::ONE << (u32::WIDTH * len))
        );
    } else {
        assert_eq!(
            n,
            Natural::from_owned_limbs_asc(xs_old) - Natural::from_limbs_asc(ys)
        );
    }
}

#[test]
fn limbs_sub_same_length_in_place_left_properties() {
    test_properties(pairs_of_unsigned_vec_var_1, |&(ref xs, ref ys)| {
        limbs_sub_in_place_left_helper(&mut limbs_sub_same_length_in_place_left, xs, ys);
    });
}

#[test]
fn limbs_sub_in_place_left_properties() {
    test_properties(pairs_of_unsigned_vec_var_3, |&(ref xs, ref ys)| {
        limbs_sub_in_place_left_helper(&mut limbs_sub_in_place_left, xs, ys);
    });
}

macro_rules! limbs_sub_in_place_right_helper {
    ($f:ident, $xs:ident, $ys:ident) => {
        |&(ref $xs, ref $ys)| {
            let mut ys = $ys.to_vec();
            let ys_old = $ys.clone();
            let len = $xs.len() as u32;
            let borrow = $f($xs, &mut ys);
            let n = Natural::from_limbs_asc(&ys);
            if borrow {
                assert_eq!(
                    n,
                    Integer::from(Natural::from_limbs_asc($xs))
                        - Integer::from(Natural::from_owned_limbs_asc(ys_old))
                        + (Integer::ONE << (u32::WIDTH * len))
                );
            } else {
                assert_eq!(
                    n,
                    Natural::from_limbs_asc($xs) - Natural::from_owned_limbs_asc(ys_old)
                );
            }
        }
    };
}

#[test]
fn limbs_sub_same_length_in_place_right_properties() {
    test_properties(
        pairs_of_unsigned_vec_var_1,
        limbs_sub_in_place_right_helper!(limbs_sub_same_length_in_place_right, xs, ys),
    );
}

#[test]
fn limbs_sub_in_place_right_properties() {
    test_properties(
        pairs_of_unsigned_vec_var_3,
        limbs_sub_in_place_right_helper!(limbs_sub_in_place_right, xs, ys),
    );
}

#[test]
fn sub_properties() {
    test_properties(pairs_of_naturals_var_1, |&(ref x, ref y)| {
        let mut mut_x = x.clone();
        mut_x -= y.clone();
        assert!(mut_x.is_valid());
        let difference = mut_x;

        let mut mut_x = x.clone();
        mut_x -= y;
        assert!(mut_x.is_valid());
        let difference_alt = mut_x;
        assert_eq!(difference_alt, difference);

        let mut rug_x = natural_to_rug_integer(x);
        rug_x -= natural_to_rug_integer(y);
        assert_eq!(rug_integer_to_natural(&rug_x), difference);

        let difference_alt = x.clone() - y.clone();
        assert!(difference_alt.is_valid());
        assert_eq!(difference_alt, difference);

        let difference_alt = x.clone() - y;
        assert_eq!(difference_alt, difference);
        assert!(difference_alt.is_valid());

        let difference_alt = x - y.clone();
        assert_eq!(difference_alt, difference);
        assert!(difference_alt.is_valid());

        let difference_alt = x - y;
        assert_eq!(difference_alt, difference);
        assert!(difference_alt.is_valid());

        assert_eq!(
            biguint_to_natural(&(natural_to_biguint(x) - natural_to_biguint(y))),
            difference
        );
        assert_eq!(
            rug_integer_to_natural(&(natural_to_rug_integer(x) - natural_to_rug_integer(y))),
            difference
        );

        assert!(difference <= *x);
        assert_eq!(difference + y, *x);
    });

    test_properties(pairs_of_natural_and_u32_var_1, |&(ref x, y)| {
        assert_eq!(x - y, x - Natural::from(y));
    });

    test_properties(pairs_of_u32_and_natural_var_1, |&(x, ref y)| {
        assert_eq!(x - y, Natural::from(x) - y);
    });

    test_properties(pairs_of_unsigneds_var_1::<u32>, |&(x, y)| {
        assert_eq!(Natural::from(x - y), Natural::from(x) - Natural::from(y));
    });

    #[allow(unknown_lints, identity_op, eq_op)]
    test_properties(naturals, |x| {
        assert_eq!(x - Natural::ZERO, *x);
        assert_eq!(x - x, Natural::ZERO);
    });
}
