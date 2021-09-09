use itertools::Itertools;
use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
use malachite_base::nevers::nevers;
use malachite_base::sets::exhaustive::lex_hash_sets_min_length;
use malachite_base::tuples::exhaustive::exhaustive_units;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

fn lex_hash_sets_min_length_helper<I: Clone + Iterator>(
    min_length: u64,
    xs: I,
    out: &[HashSet<I::Item>],
) where
    I::Item: Clone + Debug + Eq + Hash,
{
    let xss = lex_hash_sets_min_length(min_length, xs)
        .take(20)
        .collect_vec();
    assert_eq!(xss.into_iter().collect_vec().as_slice(), out);
}

fn lex_hash_sets_min_length_small_helper<I: Clone + Iterator>(
    min_length: u64,
    xs: I,
    out_len: usize,
    out: &[HashSet<I::Item>],
) where
    I::Item: Clone + Debug + Eq + Hash,
{
    let xss = lex_hash_sets_min_length(min_length, xs);
    let xss_prefix = xss.clone().take(20).collect_vec();
    assert_eq!(xss_prefix.into_iter().collect_vec().as_slice(), out);
    assert_eq!(xss.count(), out_len);
}

#[test]
fn test_lex_hash_sets_min_length() {
    lex_hash_sets_min_length_small_helper(0, nevers(), 1, &[hashset! {}]);
    lex_hash_sets_min_length_small_helper(4, nevers(), 0, &[]);
    lex_hash_sets_min_length_small_helper(0, exhaustive_units(), 2, &[hashset! {}, hashset! {()}]);
    lex_hash_sets_min_length_small_helper(5, exhaustive_units(), 0, &[]);
    lex_hash_sets_min_length_small_helper(
        0,
        exhaustive_bools(),
        4,
        &[hashset! {}, hashset! {false}, hashset! {false, true}, hashset! {true}],
    );
    lex_hash_sets_min_length_small_helper(
        1,
        exhaustive_bools(),
        3,
        &[hashset! {false}, hashset! {false, true}, hashset! {true}],
    );
    lex_hash_sets_min_length_small_helper(
        0,
        'a'..='c',
        8,
        &[
            hashset! {},
            hashset! {'a'},
            hashset! {'a', 'b'},
            hashset! {'a', 'b', 'c'},
            hashset! {'a', 'c'},
            hashset! {'b'},
            hashset! {'b', 'c'},
            hashset! {'c'},
        ],
    );
    lex_hash_sets_min_length_small_helper(
        2,
        'a'..='c',
        4,
        &[hashset! {'a', 'b'}, hashset! {'a', 'b', 'c'}, hashset! {'a', 'c'}, hashset! {'b', 'c'}],
    );
    lex_hash_sets_min_length_helper(
        0,
        exhaustive_ascii_chars(),
        &[
            hashset! {},
            hashset! {'a'},
            hashset! {'a', 'b'},
            hashset! {'a', 'b', 'c'},
            hashset! {'a', 'b', 'c', 'd'},
            hashset! {'a', 'b', 'c', 'd', 'e'},
            hashset! {'a', 'b', 'c', 'd', 'e', 'f'},
            hashset! {'a', 'b', 'c', 'd', 'e', 'f', 'g'},
            hashset! {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'},
            hashset! {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i'},
            hashset! {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'},
            hashset! {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k'},
            hashset! {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l'},
            hashset! {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm'},
            hashset! {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n'},
            hashset! {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o'},
            hashset! {
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p'
            },
            hashset! {
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q'
            },
            hashset! {
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r',
            },
            hashset! {
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's',
            },
        ],
    );
    lex_hash_sets_min_length_helper(
        3,
        exhaustive_ascii_chars(),
        &[
            hashset! {'a', 'b', 'c'},
            hashset! {'a', 'b', 'c', 'd'},
            hashset! {'a', 'b', 'c', 'd', 'e'},
            hashset! {'a', 'b', 'c', 'd', 'e', 'f'},
            hashset! {'a', 'b', 'c', 'd', 'e', 'f', 'g'},
            hashset! {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'},
            hashset! {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i'},
            hashset! {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'},
            hashset! {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k'},
            hashset! {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l'},
            hashset! {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm'},
            hashset! {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n'},
            hashset! {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o'},
            hashset! {
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p'
            },
            hashset! {
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q'
            },
            hashset! {
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r',
            },
            hashset! {
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's',
            },
            hashset! {
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's', 't',
            },
            hashset! {
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's', 't', 'u',
            },
            hashset! {
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's', 't', 'u', 'v',
            },
        ],
    );
}
