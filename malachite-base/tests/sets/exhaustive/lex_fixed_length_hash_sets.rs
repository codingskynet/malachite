use itertools::Itertools;
use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
use malachite_base::nevers::nevers;
use malachite_base::num::exhaustive::exhaustive_unsigneds;
use malachite_base::sets::exhaustive::lex_fixed_length_hash_sets;
use malachite_base::tuples::exhaustive::exhaustive_units;
use malachite_base::vecs::exhaustive::lex_fixed_length_ordered_unique_vecs;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

fn lex_fixed_length_hash_sets_helper<I: Iterator>(len: u64, xs: I, out: &[HashSet<I::Item>])
where
    I::Item: Clone + Debug + Eq + Hash,
{
    let xss = lex_fixed_length_hash_sets(len, xs).take(20).collect_vec();
    assert_eq!(xss.into_iter().collect_vec().as_slice(), out);
}

fn lex_fixed_length_hash_sets_small_helper<I: Clone + Iterator>(
    len: u64,
    xs: I,
    out_len: usize,
    out: &[HashSet<I::Item>],
) where
    I::Item: Clone + Debug + Eq + Hash,
{
    let xss = lex_fixed_length_hash_sets(len, xs);
    let xss_prefix = xss.clone().take(20).collect_vec();
    assert_eq!(xss_prefix.into_iter().collect_vec().as_slice(), out);
    assert_eq!(xss.count(), out_len);
}

#[test]
fn test_lex_fixed_length_hash_sets() {
    lex_fixed_length_hash_sets_small_helper(0, nevers(), 1, &[hashset! {}]);
    lex_fixed_length_hash_sets_small_helper(1, nevers(), 0, &[]);
    lex_fixed_length_hash_sets_small_helper(2, nevers(), 0, &[]);
    lex_fixed_length_hash_sets_small_helper(5, nevers(), 0, &[]);
    lex_fixed_length_hash_sets_small_helper(1, exhaustive_units(), 1, &[hashset! {()}]);
    lex_fixed_length_hash_sets_small_helper(2, exhaustive_units(), 0, &[]);
    lex_fixed_length_hash_sets_small_helper(5, exhaustive_units(), 0, &[]);
    lex_fixed_length_hash_sets_small_helper(0, exhaustive_unsigneds::<u8>(), 1, &[hashset! {}]);
    lex_fixed_length_hash_sets_small_helper(
        1,
        exhaustive_unsigneds::<u8>(),
        256,
        &[
            hashset! {0},
            hashset! {1},
            hashset! {2},
            hashset! {3},
            hashset! {4},
            hashset! {5},
            hashset! {6},
            hashset! {7},
            hashset! {8},
            hashset! {9},
            hashset! {10},
            hashset! {11},
            hashset! {12},
            hashset! {13},
            hashset! {14},
            hashset! {15},
            hashset! {16},
            hashset! {17},
            hashset! {18},
            hashset! {19},
        ],
    );
    lex_fixed_length_hash_sets_helper(
        1,
        exhaustive_unsigneds::<u64>(),
        &[
            hashset! {0},
            hashset! {1},
            hashset! {2},
            hashset! {3},
            hashset! {4},
            hashset! {5},
            hashset! {6},
            hashset! {7},
            hashset! {8},
            hashset! {9},
            hashset! {10},
            hashset! {11},
            hashset! {12},
            hashset! {13},
            hashset! {14},
            hashset! {15},
            hashset! {16},
            hashset! {17},
            hashset! {18},
            hashset! {19},
        ],
    );
    lex_fixed_length_hash_sets_small_helper(
        2,
        exhaustive_unsigneds::<u8>(),
        32640,
        &[
            hashset! {0, 1},
            hashset! {0, 2},
            hashset! {0, 3},
            hashset! {0, 4},
            hashset! {0, 5},
            hashset! {0, 6},
            hashset! {0, 7},
            hashset! {0, 8},
            hashset! {0, 9},
            hashset! {0, 10},
            hashset! {0, 11},
            hashset! {0, 12},
            hashset! {0, 13},
            hashset! {0, 14},
            hashset! {0, 15},
            hashset! {0, 16},
            hashset! {0, 17},
            hashset! {0, 18},
            hashset! {0, 19},
            hashset! {0, 20},
        ],
    );
    lex_fixed_length_hash_sets_helper(
        3,
        exhaustive_unsigneds::<u8>(),
        &[
            hashset! {0, 1, 2},
            hashset! {0, 1, 3},
            hashset! {0, 1, 4},
            hashset! {0, 1, 5},
            hashset! {0, 1, 6},
            hashset! {0, 1, 7},
            hashset! {0, 1, 8},
            hashset! {0, 1, 9},
            hashset! {0, 1, 10},
            hashset! {0, 1, 11},
            hashset! {0, 1, 12},
            hashset! {0, 1, 13},
            hashset! {0, 1, 14},
            hashset! {0, 1, 15},
            hashset! {0, 1, 16},
            hashset! {0, 1, 17},
            hashset! {0, 1, 18},
            hashset! {0, 1, 19},
            hashset! {0, 1, 20},
            hashset! {0, 1, 21},
        ],
    );
    lex_fixed_length_hash_sets_small_helper(
        2,
        exhaustive_ascii_chars(),
        8128,
        &[
            hashset! {'a', 'b'},
            hashset! {'a', 'c'},
            hashset! {'a', 'd'},
            hashset! {'a', 'e'},
            hashset! {'a', 'f'},
            hashset! {'a', 'g'},
            hashset! {'a', 'h'},
            hashset! {'a', 'i'},
            hashset! {'a', 'j'},
            hashset! {'a', 'k'},
            hashset! {'a', 'l'},
            hashset! {'a', 'm'},
            hashset! {'a', 'n'},
            hashset! {'a', 'o'},
            hashset! {'a', 'p'},
            hashset! {'a', 'q'},
            hashset! {'a', 'r'},
            hashset! {'a', 's'},
            hashset! {'a', 't'},
            hashset! {'a', 'u'},
        ],
    );
    lex_fixed_length_hash_sets_small_helper(
        1,
        exhaustive_bools(),
        2,
        &[hashset! {false}, hashset! {true}],
    );
    lex_fixed_length_hash_sets_small_helper(2, exhaustive_bools(), 1, &[hashset! {false, true}]);
    lex_fixed_length_hash_sets_small_helper(4, exhaustive_bools(), 0, &[]);
    lex_fixed_length_hash_sets_small_helper(
        4,
        1..=6,
        15,
        &[
            hashset! {1, 2, 3, 4},
            hashset! {1, 2, 3, 5},
            hashset! {1, 2, 3, 6},
            hashset! {1, 2, 4, 5},
            hashset! {1, 2, 4, 6},
            hashset! {1, 2, 5, 6},
            hashset! {1, 3, 4, 5},
            hashset! {1, 3, 4, 6},
            hashset! {1, 3, 5, 6},
            hashset! {1, 4, 5, 6},
            hashset! {2, 3, 4, 5},
            hashset! {2, 3, 4, 6},
            hashset! {2, 3, 5, 6},
            hashset! {2, 4, 5, 6},
            hashset! {3, 4, 5, 6},
        ],
    );
    lex_fixed_length_hash_sets_helper(
        2,
        lex_fixed_length_ordered_unique_vecs(2, exhaustive_unsigneds::<u8>()),
        &[
            hashset! {vec![0, 1], vec![0, 2]},
            hashset! {vec![0, 1], vec![0, 3]},
            hashset! {vec![0, 1], vec![0, 4]},
            hashset! {vec![0, 1], vec![0, 5]},
            hashset! {vec![0, 1], vec![0, 6]},
            hashset! {vec![0, 1], vec![0, 7]},
            hashset! {vec![0, 1], vec![0, 8]},
            hashset! {vec![0, 1], vec![0, 9]},
            hashset! {vec![0, 1], vec![0, 10]},
            hashset! {vec![0, 1], vec![0, 11]},
            hashset! {vec![0, 1], vec![0, 12]},
            hashset! {vec![0, 1], vec![0, 13]},
            hashset! {vec![0, 1], vec![0, 14]},
            hashset! {vec![0, 1], vec![0, 15]},
            hashset! {vec![0, 1], vec![0, 16]},
            hashset! {vec![0, 1], vec![0, 17]},
            hashset! {vec![0, 1], vec![0, 18]},
            hashset! {vec![0, 1], vec![0, 19]},
            hashset! {vec![0, 1], vec![0, 20]},
            hashset! {vec![0, 1], vec![0, 21]},
        ],
    );
}
