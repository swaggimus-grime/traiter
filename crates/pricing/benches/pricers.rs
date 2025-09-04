use criterion::{criterion_group, criterion_main, Criterion};
use pricing::{BlackScholes, MonteCarlo, OptionParams, OptionType, Pricer};

fn bench_black_scholes(c: &mut Criterion) {
    let p = OptionParams { s0: 100.0, k: 100.0, r: 0.05, sigma: 0.2, t: 1.0 };
    c.bench_function("Black-Scholes Call", |b| {
        b.iter(|| BlackScholes.price(p, OptionType::Call))
    });
}

fn bench_monte_carlo(c: &mut Criterion) {
    let p = OptionParams { s0: 100.0, k: 100.0, r: 0.05, sigma: 0.2, t: 1.0 };
    let mc = MonteCarlo { paths: 100_000, seed: 123 };
    c.bench_function("Monte Carlo (100k paths)", |b| {
        b.iter(|| mc.price(p, OptionType::Call))
    });
}

fn bench_monte_carlo_parallel(c: &mut Criterion) {
    let p = OptionParams { s0: 100.0, k: 100.0, r: 0.05, sigma: 0.2, t: 1.0 };
    let mc = MonteCarlo { paths: 1_000_000, seed: 123 };
    c.bench_function("Monte Carlo Parallel (1M paths)", |b| {
        b.iter(|| mc.price_parallel(p, OptionType::Call))
    });
}

criterion_group!(benches, bench_black_scholes, bench_monte_carlo, bench_monte_carlo_parallel);
criterion_main!(benches);
