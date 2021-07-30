use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::float::PrimitiveFloat;
use malachite_base_test_util::generators::{primitive_float_gen, signed_gen, unsigned_gen};
use std::cmp::Ordering;

fn primitive_sign_helper<T: PrimitiveInt>() {
    let test = |n: T, out| {
        assert_eq!(n.sign(), out);
    };
    test(T::ZERO, Ordering::Equal);
    test(T::ONE, Ordering::Greater);
    test(T::exact_from(100), Ordering::Greater);
    test(T::MAX, Ordering::Greater);
}

fn signed_sign_helper<T: PrimitiveSigned>() {
    let test = |n: T, out| {
        assert_eq!(n.sign(), out);
    };
    test(T::NEGATIVE_ONE, Ordering::Less);
    test(T::exact_from(-100), Ordering::Less);
    test(T::MIN, Ordering::Less);
}

#[test]
fn test_sign() {
    apply_fn_to_primitive_ints!(primitive_sign_helper);
    apply_fn_to_signeds!(signed_sign_helper);
}

fn sign_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_gen::<T>().test_properties(|n| {
        let sign = n.sign();
        assert_ne!(sign, Ordering::Less);
        assert_eq!(n.partial_cmp(&T::ZERO), Some(sign));
    });
}

fn sign_properties_helper_signed<T: PrimitiveSigned>() {
    signed_gen::<T>().test_properties(|n| {
        let sign = n.sign();
        assert_eq!(n.partial_cmp(&T::ZERO), Some(sign));
        if n != T::MIN {
            assert_eq!((-n).sign(), sign.reverse());
        }
    });
}

fn sign_properties_helper_primitive_float<T: PrimitiveFloat>() {
    primitive_float_gen::<T>().test_properties(|f| {
        let sign = f.sign();
        if !f.is_nan() {
            assert_eq!((-f).sign(), sign.reverse());
        }
    });
}

#[test]
fn sign_properties() {
    apply_fn_to_unsigneds!(sign_properties_helper_unsigned);
    apply_fn_to_signeds!(sign_properties_helper_signed);
    apply_fn_to_primitive_floats!(sign_properties_helper_primitive_float);
}
