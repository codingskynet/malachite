use std::str::FromStr;

use rug;

use malachite_nz::integer::Integer;

macro_rules! tests_and_properties {
    (
        $t:ident,
        $test_shl_i:ident,
        $i:ident,
        $j:ident,
        $out:ident,
        $shl_library_comparison_tests:expr,
        $n:ident
    ) => {
        #[test]
        fn $test_shl_i() {
            let test = |$i, $j: $t, $out| {
                let mut n = Integer::from_str($i).unwrap();
                n <<= $j;
                assert_eq!(n.to_string(), $out);
                assert!(n.is_valid());

                let n = Integer::from_str($i).unwrap() << $j;
                assert_eq!(n.to_string(), $out);
                assert!(n.is_valid());

                let n = &Integer::from_str($i).unwrap() << $j;
                assert_eq!(n.to_string(), $out);
                assert!(n.is_valid());

                $shl_library_comparison_tests
            };
            test("0", 0, "0");
            test("0", 10, "0");
            test("0", 10, "0");
            test("123", 1, "246");
            test("123", 2, "492");
            test("123", 25, "4127195136");
            test("123", 26, "8254390272");
            test("123", 100, "155921023828072216384094494261248");
            test("2147483648", 1, "4294967296");
            test("1000000000000", 3, "8000000000000");
            test("1000000000000", 24, "16777216000000000000");
            test("1000000000000", 25, "33554432000000000000");
            test("1000000000000", 31, "2147483648000000000000");
            test("1000000000000", 32, "4294967296000000000000");
            test("1000000000000", 33, "8589934592000000000000");
            test(
                "1000000000000",
                100,
                "1267650600228229401496703205376000000000000",
            );

            test("-123", 1, "-246");
            test("-123", 2, "-492");
            test("-123", 25, "-4127195136");
            test("-123", 26, "-8254390272");
            test("-123", 100, "-155921023828072216384094494261248");
            test("-2147483648", 1, "-4294967296");
            test("-1000000000000", 3, "-8000000000000");
            test("-1000000000000", 24, "-16777216000000000000");
            test("-1000000000000", 25, "-33554432000000000000");
            test("-1000000000000", 31, "-2147483648000000000000");
            test("-1000000000000", 32, "-4294967296000000000000");
            test("-1000000000000", 33, "-8589934592000000000000");
            test(
                "-1000000000000",
                100,
                "-1267650600228229401496703205376000000000000",
            );

            test("123", 0, "123");
            test("245", -1, "122");
            test("246", -1, "123");
            test("247", -1, "123");
            test("491", -2, "122");
            test("492", -2, "123");
            test("493", -2, "123");
            test("4127195135", -25, "122");
            test("4127195136", -25, "123");
            test("4127195137", -25, "123");
            test("8254390271", -26, "122");
            test("8254390272", -26, "123");
            test("8254390273", -26, "123");
            test("155921023828072216384094494261247", -100, "122");
            test("155921023828072216384094494261248", -100, "123");
            test("155921023828072216384094494261249", -100, "123");
            test("4294967295", -1, "2147483647");
            test("4294967296", -1, "2147483648");
            test("4294967297", -1, "2147483648");
            test("1000000000000", 0, "1000000000000");
            test("7999999999999", -3, "999999999999");
            test("8000000000000", -3, "1000000000000");
            test("8000000000001", -3, "1000000000000");
            test("16777216000000000000", -24, "1000000000000");
            test("33554432000000000000", -25, "1000000000000");
            test("2147483648000000000000", -31, "1000000000000");
            test("4294967296000000000000", -32, "1000000000000");
            test("8589934592000000000000", -33, "1000000000000");
            test(
                "1267650600228229401496703205376000000000000",
                -100,
                "1000000000000",
            );
            test("1000000000000", -10, "976562500");
            test("980657949", -72, "0");
            test("4294967295", -31, "1");
            test("4294967295", -32, "0");
            test("4294967296", -32, "1");
            test("4294967296", -33, "0");

            test("-123", 0, "-123");
            test("-245", -1, "-123");
            test("-246", -1, "-123");
            test("-247", -1, "-124");
            test("-491", -2, "-123");
            test("-492", -2, "-123");
            test("-493", -2, "-124");
            test("-4127195135", -25, "-123");
            test("-4127195136", -25, "-123");
            test("-4127195137", -25, "-124");
            test("-8254390271", -26, "-123");
            test("-8254390272", -26, "-123");
            test("-8254390273", -26, "-124");
            test("-155921023828072216384094494261247", -100, "-123");
            test("-155921023828072216384094494261248", -100, "-123");
            test("-155921023828072216384094494261249", -100, "-124");
            test("-4294967295", -1, "-2147483648");
            test("-4294967296", -1, "-2147483648");
            test("-4294967297", -1, "-2147483649");
            test("-1000000000000", 0, "-1000000000000");
            test("-7999999999999", -3, "-1000000000000");
            test("-8000000000000", -3, "-1000000000000");
            test("-8000000000001", -3, "-1000000000001");
            test("-16777216000000000000", -24, "-1000000000000");
            test("-33554432000000000000", -25, "-1000000000000");
            test("-2147483648000000000000", -31, "-1000000000000");
            test("-4294967296000000000000", -32, "-1000000000000");
            test("-8589934592000000000000", -33, "-1000000000000");
            test(
                "-1267650600228229401496703205376000000000000",
                -100,
                "-1000000000000",
            );
            test("-1000000000000", -10, "-976562500");
            test("-980657949", -72, "-1");
            test("-4294967295", -31, "-2");
            test("-4294967295", -32, "-1");
            test("-4294967296", -32, "-1");
            test("-4294967296", -33, "-1");
        }
    };
}
tests_and_properties!(i8, test_shl_i8, i, j, out, {}, n);
tests_and_properties!(i16, test_shl_i16, i, j, out, {}, n);
tests_and_properties!(
    i32,
    test_shl_i32,
    i,
    j,
    out,
    {
        let mut n = rug::Integer::from_str(i).unwrap();
        n <<= j;
        assert_eq!(n.to_string(), out);

        let n = rug::Integer::from_str(i).unwrap() << j;
        assert_eq!(n.to_string(), out);
    },
    n
);
tests_and_properties!(i64, test_shl_i64, i, j, out, {}, n);
tests_and_properties!(isize, test_shl_isize, i, j, out, {}, n);
