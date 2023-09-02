use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::from::{SignedFromFloatError, UnsignedFromFloatError};
use malachite_base::num::conversion::traits::{ConvertibleFrom, RoundingFrom};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::bench::bucketers::{
    float_complexity_bucketer, pair_1_float_complexity_bucketer,
};
use malachite_float::test_util::generators::{
    float_gen, float_rounding_mode_pair_gen_var_4, float_rounding_mode_pair_gen_var_5,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_unsigned_rounding_from_float);
    register_unsigned_demos!(runner, demo_unsigned_rounding_from_float_debug);
    register_unsigned_demos!(runner, demo_unsigned_rounding_from_float_ref);
    register_unsigned_demos!(runner, demo_unsigned_rounding_from_float_ref_debug);
    register_unsigned_demos!(runner, demo_unsigned_try_from_float);
    register_unsigned_demos!(runner, demo_unsigned_try_from_float_debug);
    register_unsigned_demos!(runner, demo_unsigned_try_from_float_ref);
    register_unsigned_demos!(runner, demo_unsigned_try_from_float_ref_debug);
    register_unsigned_demos!(runner, demo_unsigned_convertible_from_float);
    register_unsigned_demos!(runner, demo_unsigned_convertible_from_float_debug);

    register_signed_demos!(runner, demo_signed_rounding_from_float);
    register_signed_demos!(runner, demo_signed_rounding_from_float_debug);
    register_signed_demos!(runner, demo_signed_rounding_from_float_ref);
    register_signed_demos!(runner, demo_signed_rounding_from_float_ref_debug);
    register_signed_demos!(runner, demo_signed_try_from_float);
    register_signed_demos!(runner, demo_signed_try_from_float_debug);
    register_signed_demos!(runner, demo_signed_try_from_float_ref);
    register_signed_demos!(runner, demo_signed_try_from_float_ref_debug);
    register_signed_demos!(runner, demo_signed_convertible_from_float);
    register_signed_demos!(runner, demo_signed_convertible_from_float_debug);

    register_unsigned_benches!(
        runner,
        benchmark_unsigned_try_from_float_evaluation_strategy
    );
    register_unsigned_benches!(runner, benchmark_unsigned_convertible_from_float);
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_rounding_from_float_evaluation_strategy
    );
    register_signed_benches!(runner, benchmark_signed_try_from_float_evaluation_strategy);
    register_signed_benches!(runner, benchmark_signed_convertible_from_float);
    register_signed_benches!(
        runner,
        benchmark_signed_rounding_from_float_evaluation_strategy
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_unsigned_rounding_from_float<T: PrimitiveUnsigned + RoundingFrom<Float>>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: PartialOrd<T>,
    for<'a> T: ConvertibleFrom<&'a Float>,
{
    for (x, rm) in float_rounding_mode_pair_gen_var_4::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}::rounding_from({}, {}) = {:?}",
            T::NAME,
            x.clone(),
            rm,
            T::rounding_from(x, rm)
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_unsigned_rounding_from_float_debug<T: PrimitiveUnsigned + RoundingFrom<Float>>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: PartialOrd<T>,
    for<'a> T: ConvertibleFrom<&'a Float>,
{
    for (x, rm) in float_rounding_mode_pair_gen_var_4::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}::rounding_from({:#x}, {}) = {:?}",
            T::NAME,
            ComparableFloat(x.clone()),
            rm,
            T::rounding_from(x, rm)
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_unsigned_rounding_from_float_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: PartialOrd<T>,
    for<'a> T: ConvertibleFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for (x, rm) in float_rounding_mode_pair_gen_var_4::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}::rounding_from(&{}, {}) = {:?}",
            T::NAME,
            x,
            rm,
            T::rounding_from(&x, rm)
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_unsigned_rounding_from_float_ref_debug<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: PartialOrd<T>,
    for<'a> T: ConvertibleFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for (x, rm) in float_rounding_mode_pair_gen_var_4::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}::rounding_from(&{:#x}, {}) = {:?}",
            T::NAME,
            ComparableFloatRef(&x),
            rm,
            T::rounding_from(&x, rm)
        );
    }
}

fn demo_unsigned_try_from_float<
    T: TryFrom<Float, Error = UnsignedFromFloatError> + PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for x in float_gen().get(gm, config).take(limit) {
        println!(
            "{}::try_from({}) = {:?}",
            T::NAME,
            x.clone(),
            T::try_from(x)
        );
    }
}

fn demo_unsigned_try_from_float_debug<
    T: TryFrom<Float, Error = UnsignedFromFloatError> + PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for x in float_gen().get(gm, config).take(limit) {
        println!(
            "{}::try_from({:#x}) = {:?}",
            T::NAME,
            ComparableFloat(x.clone()),
            T::try_from(x)
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_unsigned_try_from_float_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> T: TryFrom<&'a Float, Error = UnsignedFromFloatError>,
{
    for x in float_gen().get(gm, config).take(limit) {
        println!("{}::try_from({}) = {:?}", T::NAME, x, T::try_from(&x));
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_unsigned_try_from_float_ref_debug<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> T: TryFrom<&'a Float, Error = UnsignedFromFloatError>,
{
    for x in float_gen().get(gm, config).take(limit) {
        println!(
            "{}::try_from({:#x}) = {:?}",
            T::NAME,
            ComparableFloatRef(&x),
            T::try_from(&x)
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_unsigned_convertible_from_float<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> T: ConvertibleFrom<&'a Float>,
{
    for f in float_gen().get(gm, config).take(limit) {
        println!(
            "{} is {}convertible to a {}",
            f,
            if T::convertible_from(&f) { "" } else { "not " },
            T::NAME
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_unsigned_convertible_from_float_debug<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> T: ConvertibleFrom<&'a Float>,
{
    for f in float_gen().get(gm, config).take(limit) {
        println!(
            "{:#x} is {}convertible to a {}",
            ComparableFloatRef(&f),
            if T::convertible_from(&f) { "" } else { "not " },
            T::NAME
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_signed_rounding_from_float<T: PrimitiveSigned + RoundingFrom<Float>>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: PartialOrd<T>,
    for<'a> T: ConvertibleFrom<&'a Float>,
{
    for (x, rm) in float_rounding_mode_pair_gen_var_5::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}::rounding_from({}, {}) = {:?}",
            T::NAME,
            x.clone(),
            rm,
            T::rounding_from(x, rm)
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_signed_rounding_from_float_debug<T: PrimitiveSigned + RoundingFrom<Float>>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: PartialOrd<T>,
    for<'a> T: ConvertibleFrom<&'a Float>,
{
    for (x, rm) in float_rounding_mode_pair_gen_var_5::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}::rounding_from({:#x}, {}) = {:?}",
            T::NAME,
            ComparableFloat(x.clone()),
            rm,
            T::rounding_from(x, rm)
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_signed_rounding_from_float_ref<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: PartialOrd<T>,
    for<'a> T: ConvertibleFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for (x, rm) in float_rounding_mode_pair_gen_var_5::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}::rounding_from(&{}, {}) = {:?}",
            T::NAME,
            x,
            rm,
            T::rounding_from(&x, rm)
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_signed_rounding_from_float_ref_debug<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: PartialOrd<T>,
    for<'a> T: ConvertibleFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for (x, rm) in float_rounding_mode_pair_gen_var_5::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}::rounding_from(&{:#x}, {}) = {:?}",
            T::NAME,
            ComparableFloatRef(&x),
            rm,
            T::rounding_from(&x, rm)
        );
    }
}

fn demo_signed_try_from_float<T: TryFrom<Float, Error = SignedFromFloatError> + PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for x in float_gen().get(gm, config).take(limit) {
        println!(
            "{}::try_from({}) = {:?}",
            T::NAME,
            x.clone(),
            T::try_from(x)
        );
    }
}

fn demo_signed_try_from_float_debug<
    T: TryFrom<Float, Error = SignedFromFloatError> + PrimitiveSigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for x in float_gen().get(gm, config).take(limit) {
        println!(
            "{}::try_from({:#x}) = {:?}",
            T::NAME,
            ComparableFloat(x.clone()),
            T::try_from(x)
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_signed_try_from_float_ref<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    for<'a> T: TryFrom<&'a Float, Error = SignedFromFloatError>,
{
    for x in float_gen().get(gm, config).take(limit) {
        println!("{}::try_from({}) = {:?}", T::NAME, x, T::try_from(&x));
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_signed_try_from_float_ref_debug<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> T: TryFrom<&'a Float, Error = SignedFromFloatError>,
{
    for x in float_gen().get(gm, config).take(limit) {
        println!(
            "{}::try_from({:#x}) = {:?}",
            T::NAME,
            ComparableFloatRef(&x),
            T::try_from(&x)
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_signed_convertible_from_float<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> T: ConvertibleFrom<&'a Float>,
{
    for f in float_gen().get(gm, config).take(limit) {
        println!(
            "{} is {}convertible to an {}",
            f,
            if T::convertible_from(&f) { "" } else { "not " },
            T::NAME
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_signed_convertible_from_float_debug<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> T: ConvertibleFrom<&'a Float>,
{
    for f in float_gen().get(gm, config).take(limit) {
        println!(
            "{:#x} is {}convertible to an {}",
            ComparableFloatRef(&f),
            if T::convertible_from(&f) { "" } else { "not " },
            T::NAME
        );
    }
}

#[allow(unused_must_use, clippy::type_repetition_in_bounds)]
fn benchmark_unsigned_try_from_float_evaluation_strategy<T: TryFrom<Float> + PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    for<'a> T: TryFrom<&'a Float>,
{
    run_benchmark(
        &format!("{}::try_from(Float)", T::NAME),
        BenchmarkType::EvaluationStrategy,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [
            (&format!("{}::try_from(Float)", T::NAME), &mut |x| {
                no_out!(T::try_from(x))
            }),
            (&format!("{}::try_from(&Float)", T::NAME), &mut |x| {
                no_out!(T::try_from(&x))
            }),
        ],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_unsigned_convertible_from_float<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    for<'a> T: ConvertibleFrom<&'a Float>,
{
    run_benchmark(
        &format!("{}::convertible_from(Float)", T::NAME),
        BenchmarkType::Single,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |x| no_out!(T::convertible_from(&x)))],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_unsigned_rounding_from_float_evaluation_strategy<
    T: RoundingFrom<Float> + PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    for<'a> T: ConvertibleFrom<&'a Float> + RoundingFrom<&'a Float>,
    Float: PartialOrd<T>,
{
    run_benchmark(
        &format!("{}::rounding_from(Float)", T::NAME),
        BenchmarkType::EvaluationStrategy,
        float_rounding_mode_pair_gen_var_4::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_float_complexity_bucketer("x"),
        &mut [
            (
                &format!("{}::rounding_from(Float)", T::NAME),
                &mut |(x, rm)| no_out!(T::rounding_from(x, rm)),
            ),
            (
                &format!("{}::rounding_from(&Float)", T::NAME),
                &mut |(x, rm)| no_out!(T::rounding_from(&x, rm)),
            ),
        ],
    );
}

#[allow(unused_must_use, clippy::type_repetition_in_bounds)]
fn benchmark_signed_try_from_float_evaluation_strategy<T: TryFrom<Float> + PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    for<'a> T: TryFrom<&'a Float>,
{
    run_benchmark(
        &format!("{}::try_from(Float)", T::NAME),
        BenchmarkType::EvaluationStrategy,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [
            (&format!("{}::try_from(Float)", T::NAME), &mut |x| {
                no_out!(T::try_from(x))
            }),
            (&format!("{}::try_from(&Float)", T::NAME), &mut |x| {
                no_out!(T::try_from(&x))
            }),
        ],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_signed_convertible_from_float<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    for<'a> T: ConvertibleFrom<&'a Float>,
{
    run_benchmark(
        &format!("{}::convertible_from(Float)", T::NAME),
        BenchmarkType::Single,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |x| no_out!(T::convertible_from(&x)))],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_signed_rounding_from_float_evaluation_strategy<
    T: RoundingFrom<Float> + PrimitiveSigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    for<'a> T: ConvertibleFrom<&'a Float> + RoundingFrom<&'a Float>,
    Float: PartialOrd<T>,
{
    run_benchmark(
        &format!("{}::rounding_from(Float)", T::NAME),
        BenchmarkType::EvaluationStrategy,
        float_rounding_mode_pair_gen_var_5::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_float_complexity_bucketer("x"),
        &mut [
            (
                &format!("{}::rounding_from(Float)", T::NAME),
                &mut |(x, rm)| no_out!(T::rounding_from(x, rm)),
            ),
            (
                &format!("{}::rounding_from(&Float)", T::NAME),
                &mut |(x, rm)| no_out!(T::rounding_from(&x, rm)),
            ),
        ],
    );
}
