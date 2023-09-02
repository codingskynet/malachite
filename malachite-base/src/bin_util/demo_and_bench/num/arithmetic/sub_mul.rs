use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::{
    triple_max_bit_bucketer, triple_max_primitive_float_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    primitive_float_triple_gen, signed_triple_gen_var_2, unsigned_triple_gen_var_2,
};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_sub_mul_unsigned);
    register_signed_demos!(runner, demo_sub_mul_signed);
    register_primitive_float_demos!(runner, demo_sub_mul_primitive_float);
    register_unsigned_demos!(runner, demo_sub_mul_assign_unsigned);
    register_signed_demos!(runner, demo_sub_mul_assign_signed);
    register_primitive_float_demos!(runner, demo_sub_mul_assign_primitive_float);

    register_unsigned_benches!(runner, benchmark_sub_mul_unsigned);
    register_signed_benches!(runner, benchmark_sub_mul_signed);
    register_primitive_float_benches!(runner, benchmark_sub_mul_primitive_float);
    register_unsigned_benches!(runner, benchmark_sub_mul_assign_unsigned);
    register_signed_benches!(runner, benchmark_sub_mul_assign_signed);
    register_primitive_float_benches!(runner, benchmark_sub_mul_assign_primitive_float);
}

fn demo_sub_mul_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in unsigned_triple_gen_var_2::<T>().get(gm, config).take(limit) {
        println!("{}.sub_mul({}, {}) = {}", x, y, z, x.sub_mul(y, z));
    }
}

fn demo_sub_mul_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in signed_triple_gen_var_2::<T>().get(gm, config).take(limit) {
        println!("({}).sub_mul({}, {}) = {}", x, y, z, x.sub_mul(y, z));
    }
}

fn demo_sub_mul_primitive_float<T: PrimitiveFloat>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in primitive_float_triple_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).sub_mul({}, {}) = {}",
            NiceFloat(x),
            NiceFloat(y),
            NiceFloat(z),
            NiceFloat(x.sub_mul(y, z))
        );
    }
}

fn demo_sub_mul_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, y, z) in unsigned_triple_gen_var_2::<T>().get(gm, config).take(limit) {
        let old_x = x;
        x.sub_mul_assign(y, z);
        println!("x := {old_x}; x.sub_mul_assign({y}, {z}); x = {x}");
    }
}

fn demo_sub_mul_assign_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, z) in signed_triple_gen_var_2::<T>().get(gm, config).take(limit) {
        let old_x = x;
        x.sub_mul_assign(y, z);
        println!("x := {old_x}; x.sub_mul_assign({y}, {z}); x = {x}");
    }
}

fn demo_sub_mul_assign_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, y, z) in primitive_float_triple_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        let old_x = x;
        x.sub_mul_assign(y, z);
        println!(
            "x := {}; x.sub_mul_assign({}, {}); x = {}",
            NiceFloat(old_x),
            NiceFloat(y),
            NiceFloat(z),
            NiceFloat(x)
        );
    }
}

fn benchmark_sub_mul_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.sub_mul({}, {})", T::NAME, T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_triple_gen_var_2::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_max_bit_bucketer("x", "y", "z"),
        &mut [("Malachite", &mut |(x, y, z)| no_out!(x.sub_mul(y, z)))],
    );
}

fn benchmark_sub_mul_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.sub_mul({}, {})", T::NAME, T::NAME, T::NAME),
        BenchmarkType::Single,
        signed_triple_gen_var_2::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_max_bit_bucketer("x", "y", "z"),
        &mut [("Malachite", &mut |(x, y, z)| no_out!(x.sub_mul(y, z)))],
    );
}

fn benchmark_sub_mul_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.sub_mul({}, {})", T::NAME, T::NAME, T::NAME),
        BenchmarkType::Single,
        primitive_float_triple_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_max_primitive_float_bucketer("x", "y", "z"),
        &mut [("Malachite", &mut |(x, y, z)| no_out!(x.sub_mul(y, z)))],
    );
}

fn benchmark_sub_mul_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.sub_mul_assign({}, {})", T::NAME, T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_triple_gen_var_2::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_max_bit_bucketer("x", "y", "z"),
        &mut [("Malachite", &mut |(mut x, y, z)| x.sub_mul_assign(y, z))],
    );
}

fn benchmark_sub_mul_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.sub_mul_assign({}, {})", T::NAME, T::NAME, T::NAME),
        BenchmarkType::Single,
        signed_triple_gen_var_2::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_max_bit_bucketer("x", "y", "z"),
        &mut [("Malachite", &mut |(mut x, y, z)| x.sub_mul_assign(y, z))],
    );
}

fn benchmark_sub_mul_assign_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.sub_mul_assign({}, {})", T::NAME, T::NAME, T::NAME),
        BenchmarkType::Single,
        primitive_float_triple_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_max_primitive_float_bucketer("x", "y", "z"),
        &mut [("Malachite", &mut |(mut x, y, z)| x.sub_mul_assign(y, z))],
    );
}
