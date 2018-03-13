use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::integer::triples_of_integer_integer_and_unsigned;
use malachite_base::num::SignificantBits;
use malachite_base::num::{SubMul, SubMulAssign};
use std::cmp::max;

pub fn demo_integer_sub_mul_assign_u32(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in triples_of_integer_integer_and_unsigned::<u32>(gm).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        a.sub_mul_assign(b, c);
        println!(
            "a := {}; x.sub_mul_assign({}, {}); x = {}",
            a_old, b_old, c, a
        );
    }
}

pub fn demo_integer_sub_mul_assign_u32_ref(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in triples_of_integer_integer_and_unsigned::<u32>(gm).take(limit) {
        let a_old = a.clone();
        a.sub_mul_assign(&b, c);
        println!("a := {}; x.sub_mul_assign(&{}, {}); x = {}", a_old, b, c, a);
    }
}

pub fn demo_integer_sub_mul_u32(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_integer_integer_and_unsigned::<u32>(gm).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        println!("{}.sub_mul({}, {}) = {}", a_old, b_old, c, a.sub_mul(b, c));
    }
}

pub fn demo_integer_sub_mul_u32_val_ref(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_integer_integer_and_unsigned::<u32>(gm).take(limit) {
        let a_old = a.clone();
        println!("{}.sub_mul(&{}, {}) = {}", a_old, b, c, a.sub_mul(&b, c));
    }
}

pub fn demo_integer_sub_mul_u32_ref_val(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_integer_integer_and_unsigned::<u32>(gm).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        println!(
            "(&{}).sub_mul({}, {}) = {}",
            a_old,
            b_old,
            c,
            (&a).sub_mul(b, c)
        );
    }
}

pub fn demo_integer_sub_mul_u32_ref_ref(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_integer_integer_and_unsigned::<u32>(gm).take(limit) {
        let a_old = a.clone();
        println!(
            "(&{}).sub_mul(&{}, {}) = {}",
            a_old,
            b,
            c,
            (&a).sub_mul(&b, c)
        );
    }
}

pub fn benchmark_integer_sub_mul_assign_u32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.sub_mul_assign(Integer, u32)",
        BenchmarkType::EvaluationStrategy,
        triples_of_integer_integer_and_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        "max(a.significant_bits(), b.significant_bits())",
        &[
            (
                "Integer.sub_mul_assign(Integer, u32)",
                &mut (|(mut a, b, c)| a.sub_mul_assign(b, c)),
            ),
            (
                "Integer.sub_mul_assign(&Integer, u32)",
                &mut (|(mut a, b, c)| a.sub_mul_assign(&b, c)),
            ),
        ],
    );
}

pub fn benchmark_integer_sub_mul_assign_u32_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.sub_mul_assign(Integer, u32)",
        BenchmarkType::Algorithms,
        triples_of_integer_integer_and_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        "max(a.significant_bits(), b.significant_bits())",
        &[
            (
                "Integer.sub_mul_assign(Integer, u32)",
                &mut (|(mut a, b, c)| a.sub_mul_assign(b, c)),
            ),
            (
                "Integer -= Integer * u32",
                &mut (|(mut a, b, c)| a -= b * c),
            ),
        ],
    );
}

pub fn benchmark_integer_sub_mul_assign_u32_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.sub_mul_assign(&Integer, u32)",
        BenchmarkType::Algorithms,
        triples_of_integer_integer_and_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        "max(a.significant_bits(), b.significant_bits())",
        &[
            (
                "Integer.sub_mul_assign(&Integer, u32)",
                &mut (|(mut a, b, c)| a.sub_mul_assign(&b, c)),
            ),
            (
                "Integer -= &Integer * u32",
                &mut (|(mut a, b, c)| a -= &b * c),
            ),
        ],
    );
}

pub fn benchmark_integer_sub_mul_u32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.sub_mul(Integer, u32)",
        BenchmarkType::EvaluationStrategy,
        triples_of_integer_integer_and_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        "max(a.significant_bits(), b.significant_bits())",
        &[
            (
                "Integer.sub_mul(Integer, u32)",
                &mut (|(a, b, c)| no_out!(a.sub_mul(b, c))),
            ),
            (
                "Integer.sub_mul(&Integer, u32)",
                &mut (|(a, b, c)| no_out!(a.sub_mul(&b, c))),
            ),
            (
                "(&Integer).sub_mul(Integer, u32)",
                &mut (|(a, b, c)| no_out!((&a).sub_mul(b, c))),
            ),
            (
                "(&Integer).sub_mul(&Integer, u32)",
                &mut (|(a, b, c)| no_out!((&a).sub_mul(&b, c))),
            ),
        ],
    );
}

pub fn benchmark_integer_sub_mul_u32_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.sub_mul(Integer, u32)",
        BenchmarkType::Algorithms,
        triples_of_integer_integer_and_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        "max(a.significant_bits(), b.significant_bits())",
        &[
            (
                "Integer.sub_mul(Integer, u32)",
                &mut (|(a, b, c)| no_out!(a.sub_mul(b, c))),
            ),
            (
                "Integer - Integer * u32",
                &mut (|(a, b, c)| no_out!(a - b * c)),
            ),
        ],
    );
}

pub fn benchmark_integer_sub_mul_u32_val_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.sub_mul(&Integer, u32)",
        BenchmarkType::Algorithms,
        triples_of_integer_integer_and_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        "max(a.significant_bits(), b.significant_bits())",
        &[
            (
                "Integer.sub_mul(&Integer, u32)",
                &mut (|(a, b, c)| no_out!(a.sub_mul(&b, c))),
            ),
            (
                "Integer - &Integer * u32",
                &mut (|(a, b, c)| no_out!(a - &b * c)),
            ),
        ],
    );
}

pub fn benchmark_integer_sub_mul_u32_ref_val_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "(&Integer).sub_mul(Integer, u32)",
        BenchmarkType::Algorithms,
        triples_of_integer_integer_and_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        "max(a.significant_bits(), b.significant_bits())",
        &[
            (
                "(&Integer).sub_mul(Integer, u32)",
                &mut (|(a, b, c)| no_out!((&a).sub_mul(b, c))),
            ),
            (
                "(&Integer) - Integer * u32",
                &mut (|(a, b, c)| no_out!(&a - b * c)),
            ),
        ],
    );
}

pub fn benchmark_integer_sub_mul_u32_ref_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "(&Integer).sub_mul(&Integer, u32)",
        BenchmarkType::Algorithms,
        triples_of_integer_integer_and_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        "max(a.significant_bits(), b.significant_bits())",
        &[
            (
                "(&Integer).sub_mul(&Integer, u32)",
                &mut (|(a, b, c)| no_out!((&a).sub_mul(&b, c))),
            ),
            (
                "(&Integer) - &Integer * u32",
                &mut (|(a, b, c)| no_out!(&a - &b * c)),
            ),
        ],
    );
}
