use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::integer::{nrm_pairs_of_integer_and_signed, pairs_of_integer_and_signed,
                      pairs_of_signed_and_integer, rm_pairs_of_integer_and_signed,
                      rm_pairs_of_signed_and_integer};
use malachite_base::num::SignificantBits;
use num::BigInt;

pub fn num_sub_i32(x: BigInt, i: i32) -> BigInt {
    x - BigInt::from(i)
}

pub fn demo_integer_sub_assign_i32(gm: GenerationMode, limit: usize) {
    for (mut n, i) in pairs_of_integer_and_signed::<i32>(gm).take(limit) {
        let n_old = n.clone();
        n -= i;
        println!("x := {}; x -= {}; x = {}", n_old, i, n);
    }
}

pub fn demo_integer_sub_i32(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_signed::<i32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} - {} = {}", n_old, i, n - i);
    }
}

pub fn demo_integer_sub_i32_ref(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_signed::<i32>(gm).take(limit) {
        println!("&{} - {} = {}", n, i, &n - i);
    }
}

pub fn demo_i32_sub_integer(gm: GenerationMode, limit: usize) {
    for (i, n) in pairs_of_signed_and_integer::<i32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} - {} = {}", i, n_old, i - n);
    }
}

pub fn demo_i32_sub_integer_ref(gm: GenerationMode, limit: usize) {
    for (i, n) in pairs_of_signed_and_integer::<i32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} - &{} = {}", i, n_old, i - &n);
    }
}

pub fn benchmark_integer_sub_assign_i32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer -= i32",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integer_and_signed::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &[
            ("malachite", &mut (|(_, (mut x, y))| x -= y)),
            ("rug", &mut (|((mut x, y), _)| x -= y)),
        ],
    );
}

pub fn benchmark_integer_sub_i32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer - i32",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_integer_and_signed(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &[
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x - y))),
            ("num", &mut (|((x, y), _, _)| no_out!(num_sub_i32(x, y)))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x - y))),
        ],
    );
}

pub fn benchmark_integer_sub_i32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer - i32",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_signed::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &[
            ("Integer - i32", &mut (|(x, y)| no_out!(x - y))),
            ("&Integer - i32", &mut (|(x, y)| no_out!(&x - y))),
        ],
    );
}

pub fn benchmark_i32_sub_integer_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "i32 - Integer",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_signed_and_integer::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, ref n))| n.significant_bits() as usize),
        "n.significant_bits()",
        &[
            ("malachite", &mut (|(_, (x, y))| no_out!(x - y))),
            ("rug", &mut (|((x, y), _)| no_out!(x - y))),
        ],
    );
}

pub fn benchmark_i32_sub_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "i32 - Integer",
        BenchmarkType::EvaluationStrategy,
        pairs_of_signed_and_integer::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &[
            ("i32 - Integer", &mut (|(x, y)| no_out!(x - y))),
            ("i32 - &Integer", &mut (|(x, y)| no_out!(x - &y))),
        ],
    );
}
