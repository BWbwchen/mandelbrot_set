use criterion::*;

use mandelbrot_set::seq::Seq;
use mandelbrot_set::MandelbrotSet;
use mandelbrot_set::Setting;

static SETTING_SMALL_CASE: Setting = Setting {
    output_file: String::new(),
    iteration: 100,
    real_lower: -2.0,
    real_upper: 1.0,
    imag_lower: 1.0,
    imag_upper: -1.0,
    img_width: 1200,
    img_height: 800,
};
static SETTING_MID_CASE: Setting = Setting {
    output_file: String::new(),
    iteration: 100,
    real_lower: -2.0,
    real_upper: 1.0,
    imag_lower: 1.0,
    imag_upper: -1.0,
    img_width: 4800,
    img_height: 3200,
};
static SETTING_BIG_CASE: Setting = Setting {
    output_file: String::new(),
    iteration: 100,
    real_lower: -2.0,
    real_upper: 1.0,
    imag_lower: 1.0,
    imag_upper: -1.0,
    img_width: 12000,
    img_height: 8000,
};

fn benchmark_seq(c: &mut Criterion) {
    let execute = Seq;
    let mut group = c.benchmark_group("Seq Benchmark");
    group.sampling_mode(SamplingMode::Flat).sample_size(10);
    group.bench_function("seq small case", |b| {
        b.iter(|| execute.calculate(black_box(&SETTING_SMALL_CASE)))
    });
    // .bench_function("seq mid case", |b| {
    //     b.iter(|| execute.calculate(black_box(&SETTING_MID_CASE)))
    // })
    // .bench_function("seq big case", |b| {
    //     b.iter(|| execute.calculate(black_box(&SETTING_BIG_CASE)))
    // });

    group.finish();
}

criterion_group!(benches, benchmark_seq);
criterion_main!(benches);
