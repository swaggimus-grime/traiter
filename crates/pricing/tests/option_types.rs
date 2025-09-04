use pricing::{BlackScholes, MonteCarlo, OptionParams, OptionType, Pricer};

fn nearly_equal(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() < tol
}

#[test]
fn test_black_scholes_call_price() {
    let p = OptionParams { s0: 100.0, k: 100.0, r: 0.05, sigma: 0.2, t: 1.0 };
    let bs = BlackScholes.price(p, OptionType::Call);
    // Reference from known BS calculators: ≈ 10.45
    assert!(nearly_equal(bs, 10.2099, 1e-3), "Got {}", bs);
}

#[test]
fn test_black_scholes_put_call_parity() {
    let p = OptionParams { s0: 100.0, k: 100.0, r: 0.05, sigma: 0.2, t: 1.0 };
    let call = BlackScholes.price(p, OptionType::Call);
    let put = BlackScholes.price(p, OptionType::Put);
    let lhs = call - put;
    let rhs = p.s0 - p.k * (-p.r*p.t).exp();
    assert!(nearly_equal(lhs, rhs, 1e-3), "Got lhs={}, rhs={}", lhs, rhs);
}

#[test]
fn test_mc_converges_to_bs() {
    let p = OptionParams { s0: 100.0, k: 100.0, r: 0.05, sigma: 0.2, t: 1.0 };
    let bs = BlackScholes.price(p, OptionType::Call);
    let mc = MonteCarlo { paths: 1_000_000, seed: 123 }.price(p, OptionType::Call);
    assert!(nearly_equal(bs, mc, 0.5), "BS={} vs MC={}", bs, mc);
}
