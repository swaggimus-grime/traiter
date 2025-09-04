use pricing::{BlackScholes, MonteCarlo, OptionParams, OptionType, Pricer};

#[test]
fn test_black_scholes_greeks() {
    let bs = BlackScholes;
    let p = OptionParams { s0: 100.0, k: 100.0, r: 0.05, sigma: 0.2, t: 1.0 };

    let delta_call = bs.delta(p, OptionType::Call);
    let delta_put  = bs.delta(p, OptionType::Put);
    let gamma      = bs.gamma(p, OptionType::Call);
    let vega       = bs.vega(p, OptionType::Call);
    let theta_call = bs.theta(p, OptionType::Call);
    let theta_put  = bs.theta(p, OptionType::Put);
    let rho_call   = bs.rho(p, OptionType::Call);
    let rho_put    = bs.rho(p, OptionType::Put);

    let tol = 1e-9; // analytic formulas → very tight

    assert!((delta_call - 0.636830651175619).abs() < tol);
    assert!((delta_put  - (-0.363169348824381)).abs() < tol);
    assert!((gamma      - 0.018762017345846895).abs() < tol);
    assert!((vega       - 37.52403469169379).abs() < tol);
    assert!((theta_call - (-6.414027546438197)).abs() < tol);
    assert!((theta_put  - (-1.657880423934626)).abs() < tol);
    assert!((rho_call   - 53.232481545376345).abs() < tol);
    assert!((rho_put    - (-41.89046090469506)).abs() < tol);
}

#[test]
fn test_mc_greeks_vs_bs() {
    let bs = BlackScholes;
    let mc = MonteCarlo { paths: 200_000, seed: 42 };
    let p = OptionParams { s0: 100.0, k: 100.0, r: 0.05, sigma: 0.2, t: 1.0 };

    let delta_bs = bs.delta(p, OptionType::Call);
    let delta_mc = mc.delta(p, OptionType::Call);
    println!("Delta BS = {}, MC = {}", delta_bs, delta_mc);
    assert!((delta_bs - delta_mc).abs() < 1e-2);

    let gamma_bs = bs.gamma(p, OptionType::Call);
    let gamma_mc = mc.gamma(p, OptionType::Call);
    println!("Gamma BS = {}, MC = {}", gamma_bs, gamma_mc);
    assert!((gamma_bs - gamma_mc).abs() < 1e-2);

    let vega_bs = bs.vega(p, OptionType::Call);
    let vega_mc = mc.vega(p, OptionType::Call);
    println!("Vega BS = {}, MC = {}", vega_bs, vega_mc);
    assert!((vega_bs - vega_mc).abs() < 1e-1);

    let rho_bs = bs.rho(p, OptionType::Call);
    let rho_mc = mc.rho(p, OptionType::Call);
    println!("Rho BS = {}, MC = {}", rho_bs, rho_mc);
    assert!((rho_bs - rho_mc).abs() < 1.0);

    let theta_bs = bs.theta(p, OptionType::Call);
    let theta_mc = mc.theta(p, OptionType::Call);
    println!("Theta BS = {}, MC = {}", theta_bs, theta_mc);
    assert!((theta_bs - theta_mc).abs() < 0.5);
}
