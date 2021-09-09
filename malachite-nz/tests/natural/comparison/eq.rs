use malachite_base_test_util::common::test_eq_helper;
use malachite_base_test_util::generators::unsigned_pair_gen_var_27;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz_test_util::common::{natural_to_biguint, natural_to_rug_integer};
use malachite_nz_test_util::generators::{natural_gen, natural_pair_gen, natural_triple_gen};
use num::BigUint;
use rug;

#[test]
fn test_eq() {
    let strings = vec!["0", "1", "2", "123", "1000000000000"];
    test_eq_helper::<Natural>(&strings);
    test_eq_helper::<BigUint>(&strings);
    test_eq_helper::<rug::Integer>(&strings);
}

#[allow(clippy::cmp_owned, clippy::eq_op)]
#[test]
fn eq_properties() {
    natural_pair_gen().test_properties(|(x, y)| {
        let eq = x == y;
        assert_eq!(natural_to_biguint(&x) == natural_to_biguint(&y), eq);
        assert_eq!(natural_to_rug_integer(&x) == natural_to_rug_integer(&y), eq);
        assert_eq!(y == x, eq);
    });

    natural_gen().test_properties(|x| {
        assert_eq!(x, x);
    });

    natural_triple_gen().test_properties(|(x, y, z)| {
        if x == y && y == z {
            assert_eq!(x, z);
        }
    });

    unsigned_pair_gen_var_27::<Limb>().test_properties(|(x, y)| {
        assert_eq!(Natural::from(x) == Natural::from(y), x == y);
    });
}
