use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{pairs_of_u32_vec_and_u32_var_1,
                   triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1};
use inputs::integer::{pairs_of_integer_and_signed, pairs_of_signed_and_integer,
                      rm_pairs_of_integer_and_signed, rm_pairs_of_signed_and_integer};
use malachite_base::num::SignificantBits;
use malachite_nz::integer::logic::and_i32::{limbs_slice_neg_and_limb_neg,
                                            limbs_slice_neg_and_limb_neg_in_place,
                                            limbs_vec_neg_and_limb_neg,
                                            limbs_vec_neg_and_limb_neg_in_place};
use malachite_nz::integer::Integer;
use std::iter::repeat;
use std::u32;

pub fn integer_and_i32_alt_1(n: &Integer, i: i32) -> Integer {
    let n_negative = *n < 0;
    let i_negative = i < 0;
    let i = Integer::from(i);
    let bit_zip: Box<Iterator<Item = (bool, bool)>> =
        if n.significant_bits() >= i.significant_bits() {
            Box::new(
                n.twos_complement_bits()
                    .zip(i.twos_complement_bits().chain(repeat(i_negative))),
            )
        } else {
            Box::new(
                n.twos_complement_bits()
                    .chain(repeat(n_negative))
                    .zip(i.twos_complement_bits()),
            )
        };
    let mut and_bits = Vec::new();
    for (b, c) in bit_zip {
        and_bits.push(b && c);
    }
    and_bits.push(n_negative && i_negative);
    Integer::from_twos_complement_bits_asc(&and_bits)
}

pub fn integer_and_i32_alt_2(n: &Integer, i: i32) -> Integer {
    let n_extension = if *n < 0 { u32::MAX } else { 0 };
    let i_extension = if i < 0 { u32::MAX } else { 0 };
    let i = Integer::from(i);
    let limb_zip: Box<Iterator<Item = (u32, u32)>> =
        if n.twos_complement_limbs().count() >= i.twos_complement_limbs().count() {
            Box::new(
                n.twos_complement_limbs()
                    .zip(i.twos_complement_limbs().chain(repeat(i_extension))),
            )
        } else {
            Box::new(
                n.twos_complement_limbs()
                    .chain(repeat(n_extension))
                    .zip(i.twos_complement_limbs()),
            )
        };
    let mut and_limbs = Vec::new();
    for (x, y) in limb_zip {
        and_limbs.push(x & y);
    }
    and_limbs.push(n_extension & i_extension);
    Integer::from_owned_twos_complement_limbs_asc(and_limbs)
}

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_slice_neg_and_limb_neg);
    register_demo!(registry, demo_limbs_vec_neg_and_limb_neg);
    register_demo!(registry, demo_limbs_slice_neg_and_limb_neg_in_place);
    register_demo!(registry, demo_limbs_vec_neg_and_limb_neg_in_place);
    register_demo!(registry, demo_integer_and_assign_i32);
    register_demo!(registry, demo_integer_and_i32);
    register_demo!(registry, demo_integer_and_i32_ref);
    register_demo!(registry, demo_i32_and_integer);
    register_demo!(registry, demo_i32_and_integer_ref);
    register_bench!(registry, Small, benchmark_limbs_slice_neg_and_limb_neg);
    register_bench!(registry, Small, benchmark_limbs_vec_neg_and_limb_neg);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_slice_neg_and_limb_neg_in_place
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_vec_neg_and_limb_neg_in_place
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_and_assign_i32_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_and_i32_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_and_i32_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_integer_and_i32_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_i32_and_integer_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_i32_and_integer_evaluation_strategy
    );
}

fn demo_limbs_slice_neg_and_limb_neg(gm: GenerationMode, limit: usize) {
    for (out_limbs, in_limbs, limb) in
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1(gm).take(limit)
    {
        let mut out_limbs = out_limbs.to_vec();
        let mut out_limbs_old = out_limbs.clone();
        let carry = limbs_slice_neg_and_limb_neg(&mut out_limbs, &in_limbs, limb);
        println!(
            "out_limbs := {:?}; limbs_slice_neg_and_limb_neg(&mut out_limbs, {:?}, {}) = {}; out_limbs = {:?}",
            out_limbs_old, in_limbs, limb, carry, out_limbs
        );
    }
}

fn demo_limbs_vec_neg_and_limb_neg(gm: GenerationMode, limit: usize) {
    for (limbs, limb) in pairs_of_u32_vec_and_u32_var_1(gm).take(limit) {
        println!(
            "limbs_vec_neg_and_limb_neg({:?}, {}) = {:?}",
            limbs,
            limb,
            limbs_vec_neg_and_limb_neg(&limbs, limb)
        );
    }
}

fn demo_limbs_slice_neg_and_limb_neg_in_place(gm: GenerationMode, limit: usize) {
    for (limbs, limb) in pairs_of_u32_vec_and_u32_var_1(gm).take(limit) {
        let mut limbs = limbs.to_vec();
        let mut limbs_old = limbs.clone();
        let carry = limbs_slice_neg_and_limb_neg_in_place(&mut limbs, limb);
        println!(
            "limbs := {:?}; limbs_slice_neg_and_limb_neg_in_place(&mut limbs, {}) = {}; limbs = {:?}",
            limbs_old, limb, carry, limbs
        );
    }
}

fn demo_limbs_vec_neg_and_limb_neg_in_place(gm: GenerationMode, limit: usize) {
    for (limbs, limb) in pairs_of_u32_vec_and_u32_var_1(gm).take(limit) {
        let mut limbs = limbs.to_vec();
        let mut limbs_old = limbs.clone();
        limbs_vec_neg_and_limb_neg_in_place(&mut limbs, limb);
        println!(
            "limbs := {:?}; limbs_vec_neg_and_limb_neg_in_place(&mut limbs, {}); limbs = {:?}",
            limbs_old, limb, limbs
        );
    }
}

fn demo_integer_and_assign_i32(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_integer_and_signed::<i32>(gm).take(limit) {
        let n_old = n.clone();
        n &= u;
        println!("x := {}; x &= {}; x = {}", n_old, u, n);
    }
}

fn demo_integer_and_i32(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_signed::<i32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} & {} = {}", n_old, u, n & u);
    }
}

fn demo_integer_and_i32_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_signed::<i32>(gm).take(limit) {
        println!("&{} & {} = {}", n, u, &n & u);
    }
}

fn demo_i32_and_integer(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_signed_and_integer::<i32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} + {} = {}", u, n_old, u & n);
    }
}

fn demo_i32_and_integer_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_signed_and_integer::<i32>(gm).take(limit) {
        println!("{} + &{} = {}", u, n, u & &n);
    }
}

fn benchmark_limbs_slice_neg_and_limb_neg(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_slice_neg_and_limb_neg(&mut [u32], &[u32], u32)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref in_limbs, _)| in_limbs.len()),
        "in_limbs.len()",
        &mut [(
            "malachite",
            &mut (|(mut out_limbs, in_limbs, limb)| {
                no_out!(limbs_slice_neg_and_limb_neg(
                    &mut out_limbs,
                    &in_limbs,
                    limb
                ))
            }),
        )],
    );
}

fn benchmark_limbs_vec_neg_and_limb_neg(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_vec_neg_and_limb_neg(&[u32], u32)",
        BenchmarkType::Single,
        pairs_of_u32_vec_and_u32_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(limbs, limb)| no_out!(limbs_vec_neg_and_limb_neg(&limbs, limb))),
        )],
    );
}

fn benchmark_limbs_slice_neg_and_limb_neg_in_place(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_slice_neg_and_limb_neg_in_place(&mut [u32], u32)",
        BenchmarkType::Single,
        pairs_of_u32_vec_and_u32_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(mut limbs, limb)| {
                no_out!(limbs_slice_neg_and_limb_neg_in_place(&mut limbs, limb))
            }),
        )],
    );
}

fn benchmark_limbs_vec_neg_and_limb_neg_in_place(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_vec_neg_and_limb_neg_in_place(&Vec[u32], u32)",
        BenchmarkType::Single,
        pairs_of_u32_vec_and_u32_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(mut limbs, limb)| limbs_vec_neg_and_limb_neg_in_place(&mut limbs, limb)),
        )],
    );
}

fn benchmark_integer_and_assign_i32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer &= i32",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integer_and_signed::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (mut x, y))| x &= y)),
            ("rug", &mut (|((mut x, y), _)| x &= y)),
        ],
    );
}

fn benchmark_integer_and_i32_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer & i32",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integer_and_signed::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(&x & y))),
            ("rug", &mut (|((x, y), _)| no_out!(x & y))),
        ],
    );
}

fn benchmark_integer_and_i32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer & i32",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_signed(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("Integer & i32", &mut (|(x, y)| no_out!(x & y))),
            ("&Integer & i32", &mut (|(x, y)| no_out!(&x & y))),
        ],
    );
}

fn benchmark_integer_and_i32_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer & i32",
        BenchmarkType::LibraryComparison,
        pairs_of_integer_and_signed(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("default", &mut (|(x, y)| no_out!(&x & y))),
            (
                "using bits explicitly",
                &mut (|(x, y)| no_out!(integer_and_i32_alt_1(&x, y))),
            ),
            (
                "using limbs explicitly",
                &mut (|(x, y)| no_out!(integer_and_i32_alt_2(&x, y))),
            ),
        ],
    );
}

fn benchmark_i32_and_integer_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "i32 & Integer",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_signed_and_integer::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, ref n))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x & &y))),
            ("rug", &mut (|((x, y), _)| no_out!(x & y))),
        ],
    );
}

fn benchmark_i32_and_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "i32 & Integer",
        BenchmarkType::EvaluationStrategy,
        pairs_of_signed_and_integer::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("i32 & Integer", &mut (|(x, y)| no_out!(x & y))),
            ("i32 & &Integer", &mut (|(x, y)| no_out!(x & &y))),
        ],
    );
}
