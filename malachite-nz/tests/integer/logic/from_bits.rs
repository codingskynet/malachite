use malachite_base::num::logic::traits::BitConvertible;

use malachite_nz::integer::Integer;

#[test]
fn test_from_bits_asc() {
    let test = |bits: &[bool], out| {
        let x = Integer::from_bits_asc(bits);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
    };
    test(&[], "0");
    test(&[false], "0");
    test(&[false, false, false], "0");
    test(&[true, false], "1");
    test(&[false, true, true, false], "6");
    test(&[true, false, false, true, false, true, true, false], "105");
    test(
        &[true, false, false, true, false, true, true, false, false],
        "105",
    );
    test(
        &[
            true, false, false, true, false, true, true, false, false, false,
        ],
        "105",
    );
    test(
        &[
            false, false, false, false, false, false, false, false, false, false, false, false,
            true, false, false, false, true, false, true, false, false, true, false, true, false,
            false, true, false, true, false, true, true, false, false, false, true, false, true,
            true, true, false,
        ],
        "1000000000000",
    );
    test(
        &[
            false, false, false, false, false, false, false, false, false, false, false, false,
            true, false, false, false, true, false, true, false, false, true, false, true, false,
            false, true, false, true, false, true, true, false, false, false, true, false, true,
            true, true, false, false,
        ],
        "1000000000000",
    );
}

#[test]
fn test_from_bits_desc() {
    let test = |bits: &[bool], out| {
        let x = Integer::from_bits_desc(bits);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
    };
    test(&[], "0");
    test(&[false], "0");
    test(&[false, false, false], "0");
    test(&[false, true], "1");
    test(&[false, true, true, false], "6");
    test(&[false, true, true, false, true, false, false, true], "105");
    test(
        &[false, false, true, true, false, true, false, false, true],
        "105",
    );
    test(
        &[
            false, false, false, true, true, false, true, false, false, true,
        ],
        "105",
    );
    test(
        &[
            false, true, true, true, false, true, false, false, false, true, true, false, true,
            false, true, false, false, true, false, true, false, false, true, false, true, false,
            false, false, true, false, false, false, false, false, false, false, false, false,
            false, false, false,
        ],
        "1000000000000",
    );
    test(
        &[
            false, false, true, true, true, false, true, false, false, false, true, true, false,
            true, false, true, false, false, true, false, true, false, false, true, false, true,
            false, false, false, true, false, false, false, false, false, false, false, false,
            false, false, false, false,
        ],
        "1000000000000",
    );
}
