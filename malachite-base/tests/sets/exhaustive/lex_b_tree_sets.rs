use itertools::Itertools;
use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
use malachite_base::nevers::nevers;
use malachite_base::sets::exhaustive::lex_b_tree_sets;
use malachite_base::tuples::exhaustive::exhaustive_units;
use std::collections::BTreeSet;
use std::fmt::Debug;

fn lex_b_tree_sets_helper<I: Clone + Iterator>(xs: I, out: &[BTreeSet<I::Item>])
where
    I::Item: Clone + Debug + Eq + Ord,
{
    let xss = lex_b_tree_sets(xs).take(20).collect_vec();
    assert_eq!(xss.into_iter().collect_vec().as_slice(), out);
}

fn lex_b_tree_sets_small_helper<I: Clone + Iterator>(
    xs: I,
    out_len: usize,
    out: &[BTreeSet<I::Item>],
) where
    I::Item: Clone + Debug + Eq + Ord,
{
    let xss = lex_b_tree_sets(xs);
    let xss_prefix = xss.clone().take(20).collect_vec();
    assert_eq!(xss_prefix.into_iter().collect_vec().as_slice(), out);
    assert_eq!(xss.count(), out_len);
}

#[test]
fn test_lex_b_tree_sets() {
    lex_b_tree_sets_small_helper(nevers(), 1, &[btreeset! {}]);
    lex_b_tree_sets_small_helper(exhaustive_units(), 2, &[btreeset! {}, btreeset! {()}]);
    lex_b_tree_sets_small_helper(
        exhaustive_bools(),
        4,
        &[btreeset! {}, btreeset! {false}, btreeset! {false, true}, btreeset! {true}],
    );
    lex_b_tree_sets_small_helper(
        1..=6,
        64,
        &[
            btreeset! {},
            btreeset! {1},
            btreeset! {1, 2},
            btreeset! {1, 2, 3},
            btreeset! {1, 2, 3, 4},
            btreeset! {1, 2, 3, 4, 5},
            btreeset! {1, 2, 3, 4, 5, 6},
            btreeset! {1, 2, 3, 4, 6},
            btreeset! {1, 2, 3, 5},
            btreeset! {1, 2, 3, 5, 6},
            btreeset! {1, 2, 3, 6},
            btreeset! {1, 2, 4},
            btreeset! {1, 2, 4, 5},
            btreeset! {1, 2, 4, 5, 6},
            btreeset! {1, 2, 4, 6},
            btreeset! {1, 2, 5},
            btreeset! {1, 2, 5, 6},
            btreeset! {1, 2, 6},
            btreeset! {1, 3},
            btreeset! {1, 3, 4},
        ],
    );
    lex_b_tree_sets_small_helper(
        'a'..='c',
        8,
        &[
            btreeset! {},
            btreeset! {'a'},
            btreeset! {'a', 'b'},
            btreeset! {'a', 'b', 'c'},
            btreeset! {'a', 'c'},
            btreeset! {'b'},
            btreeset! {'b', 'c'},
            btreeset! {'c'},
        ],
    );
    lex_b_tree_sets_helper(
        exhaustive_ascii_chars(),
        &[
            btreeset! {},
            btreeset! {'a'},
            btreeset! {'a', 'b'},
            btreeset! {'a', 'b', 'c'},
            btreeset! {'a', 'b', 'c', 'd'},
            btreeset! {'a', 'b', 'c', 'd', 'e'},
            btreeset! {'a', 'b', 'c', 'd', 'e', 'f'},
            btreeset! {'a', 'b', 'c', 'd', 'e', 'f', 'g'},
            btreeset! {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'},
            btreeset! {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i'},
            btreeset! {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'},
            btreeset! {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k'},
            btreeset! {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l'},
            btreeset! {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm'},
            btreeset! {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n'},
            btreeset! {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o'},
            btreeset! {
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p'
            },
            btreeset! {
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q'
            },
            btreeset! {
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r',
            },
            btreeset! {
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's',
            },
        ],
    );
}
