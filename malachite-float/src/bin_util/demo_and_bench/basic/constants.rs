use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::bench::bucketers::unsigned_direct_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_gen_var_11;
use malachite_base::test_util::runner::Runner;
use malachite_float::ComparableFloat;
use malachite_float::Float;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_one_prec);
    register_demo!(runner, demo_float_one_prec_debug);
    register_demo!(runner, demo_float_two_prec);
    register_demo!(runner, demo_float_two_prec_debug);
    register_demo!(runner, demo_float_negative_one_prec);
    register_demo!(runner, demo_float_negative_one_prec_debug);
    register_demo!(runner, demo_float_one_half_prec);
    register_demo!(runner, demo_float_one_half_prec_debug);

    register_bench!(runner, benchmark_float_one_prec_library_comparison);
    register_bench!(runner, benchmark_float_two_prec_library_comparison);
    register_bench!(runner, benchmark_float_negative_one_prec_library_comparison);
    register_bench!(runner, benchmark_float_one_half_prec_library_comparison);
}

fn demo_float_one_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in unsigned_gen_var_11().get(gm, config).take(limit) {
        println!("one_prec({}) = {}", p, Float::one_prec(p));
    }
}

fn demo_float_one_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in unsigned_gen_var_11().get(gm, config).take(limit) {
        println!(
            "one_prec({}) = {:#x}",
            p,
            ComparableFloat(Float::one_prec(p))
        );
    }
}

fn demo_float_two_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in unsigned_gen_var_11().get(gm, config).take(limit) {
        println!("two_prec({}) = {}", p, Float::two_prec(p));
    }
}

fn demo_float_two_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in unsigned_gen_var_11().get(gm, config).take(limit) {
        println!(
            "two_prec({}) = {:#x}",
            p,
            ComparableFloat(Float::two_prec(p))
        );
    }
}

fn demo_float_negative_one_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in unsigned_gen_var_11().get(gm, config).take(limit) {
        println!("negative_one_prec({}) = {}", p, Float::negative_one_prec(p));
    }
}

fn demo_float_negative_one_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in unsigned_gen_var_11().get(gm, config).take(limit) {
        println!(
            "negative_one_prec({}) = {:#x}",
            p,
            ComparableFloat(Float::negative_one_prec(p))
        );
    }
}

fn demo_float_one_half_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in unsigned_gen_var_11().get(gm, config).take(limit) {
        println!("one_half_prec({}) = {}", p, Float::one_half_prec(p));
    }
}

fn demo_float_one_half_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in unsigned_gen_var_11().get(gm, config).take(limit) {
        println!(
            "one_half_prec({}) = {:#x}",
            p,
            ComparableFloat(Float::one_half_prec(p))
        );
    }
}

fn benchmark_float_one_prec_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.one_prec()",
        BenchmarkType::LibraryComparison,
        unsigned_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("Malachite", &mut |p| no_out!(Float::one_prec(p))),
            ("rug", &mut |p| {
                no_out!(rug::Float::with_val(u32::exact_from(p), 1.0))
            }),
        ],
    );
}

fn benchmark_float_two_prec_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.two_prec()",
        BenchmarkType::LibraryComparison,
        unsigned_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("Malachite", &mut |p| no_out!(Float::two_prec(p))),
            ("rug", &mut |p| {
                no_out!(rug::Float::with_val(u32::exact_from(p), 2.0))
            }),
        ],
    );
}

fn benchmark_float_negative_one_prec_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.negative_one_prec()",
        BenchmarkType::LibraryComparison,
        unsigned_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("Malachite", &mut |p| no_out!(Float::negative_one_prec(p))),
            ("rug", &mut |p| {
                no_out!(rug::Float::with_val(u32::exact_from(p), -1.0))
            }),
        ],
    );
}

fn benchmark_float_one_half_prec_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.one_half_prec()",
        BenchmarkType::LibraryComparison,
        unsigned_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("Malachite", &mut |p| no_out!(Float::one_half_prec(p))),
            ("rug", &mut |p| {
                no_out!(rug::Float::with_val(u32::exact_from(p), 0.5))
            }),
        ],
    );
}
