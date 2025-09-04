use rayon::iter::ParallelIterator;
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand_distr::{StandardNormal, Distribution};
use rayon::prelude::IntoParallelIterator;
use crate::option::{payoff, OptionParams, OptionType};
use crate::pricers::Pricer;

pub struct MonteCarlo {
    pub paths: usize,
    pub seed: u64,
}

impl MonteCarlo {
    pub fn new(paths: usize, seed: u64) -> Self {
        MonteCarlo { paths, seed }
    }

    fn price_with_seed(&self, p: OptionParams, opt: OptionType, seed: u64) -> f64 {
        let mut rng = StdRng::seed_from_u64(seed);
        let normal = StandardNormal;
        let drift = (p.r - 0.5 * p.sigma * p.sigma) * p.t;
        let diff = p.sigma * p.t.sqrt();
        let mut sum = 0.0;

        for _ in 0..self.paths {
            let z: f64 = normal.sample(&mut rng);
            let st = p.s0 * ((drift + diff * z).exp());
            sum += payoff(opt, st, p.k);
        }

        (-(p.r * p.t)).exp() * sum / self.paths as f64
    }

    pub fn delta_pathwise(&self, p: OptionParams, opt: OptionType) -> f64 {
        let mut rng = StdRng::seed_from_u64(self.seed);
        let normal = StandardNormal;
        let drift = (p.r - 0.5 * p.sigma * p.sigma) * p.t;
        let diff = p.sigma * p.t.sqrt();
        let mut sum = 0.0;

        for _ in 0..self.paths {
            let z: f64 = normal.sample(&mut rng);
            let st = p.s0 * (drift + diff * z).exp();
            match opt {
                OptionType::Call => {
                    if st > p.k {
                        sum += st / p.s0;
                    }
                }
                OptionType::Put => {
                    if st < p.k {
                        sum += -st / p.s0;
                    }
                }
            }
        }

        (-p.r * p.t).exp() * sum / self.paths as f64
    }

    pub fn vega_pathwise(&self, p: OptionParams, opt: OptionType) -> f64 {
        let mut rng = StdRng::seed_from_u64(self.seed);
        let normal = StandardNormal;
        let drift = (p.r - 0.5 * p.sigma * p.sigma) * p.t;
        let diff = p.sigma * p.t.sqrt();
        let mut sum = 0.0;

        for _ in 0..self.paths {
            let z: f64 = normal.sample(&mut rng);
            let st = p.s0 * (drift + diff * z).exp();
            let adj = st * (-p.sigma * p.t + p.t.sqrt() * z);

            match opt {
                OptionType::Call => {
                    if st > p.k {
                        sum += adj;
                    }
                }
                OptionType::Put => {
                    if st < p.k {
                        sum += -adj;
                    }
                }
            }
        }

        (-p.r * p.t).exp() * sum / self.paths as f64
    }
}

impl Pricer for MonteCarlo {
    fn price(&self, p: OptionParams, opt: OptionType) -> f64 {
        p.validate().expect("invalid params");

        let mut rng = StdRng::seed_from_u64(self.seed);
        let normal = StandardNormal;
        let drift = (p.r - 0.5*p.sigma*p.sigma) * p.t;
        let diff  = p.sigma * p.t.sqrt();
        let mut sum = 0.0;

        for _ in 0..self.paths {
            let z: f64 = normal.sample(&mut rng);
            let st = p.s0 * f64::exp(drift + diff*z);
            sum += payoff(opt, st, p.k);
        }


        (-(p.r*p.t)).exp() * sum / self.paths as f64
    }

    fn price_parallel(&self, p: OptionParams, opt: OptionType) -> f64 {
        let discount = (-p.r * p.t).exp();
        let dt = p.t;

        let payoffs: f64 = (0..self.paths)
            .into_par_iter()
            .map(|i| {
                use rand::prelude::*;
                let mut rng = StdRng::seed_from_u64(self.seed + i as u64);
                let z: f64 = rng.sample(rand_distr::StandardNormal);
                let st = p.s0 * ( (p.r - 0.5 * p.sigma * p.sigma) * dt + p.sigma * z * dt.sqrt() ).exp();
                match opt {
                    OptionType::Call => (st - p.k).max(0.0),
                    OptionType::Put => (p.k - st).max(0.0),
                }
            })
            .sum();

        discount * payoffs / self.paths as f64
    }

    fn delta(&self, p: OptionParams, opt: OptionType) -> f64 {
        let h = 0.01 * p.s0; // bump size = 1% of S0
        let seed = self.seed;
        let up = self.price_with_seed(OptionParams { s0: p.s0 + h, ..p }, opt, seed);
        let down = self.price_with_seed(OptionParams { s0: p.s0 - h, ..p }, opt, seed);
        (up - down) / (2.0 * h)
    }

    fn gamma(&self, p: OptionParams, opt: OptionType) -> f64 {
        let h = 0.01 * p.s0;
        let seed = self.seed;
        let up = self.price_with_seed(OptionParams { s0: p.s0 + h, ..p }, opt, seed);
        let mid = self.price_with_seed(p, opt, seed);
        let down = self.price_with_seed(OptionParams { s0: p.s0 - h, ..p }, opt, seed);
        (up - 2.0 * mid + down) / (h * h)
    }

    fn vega(&self, p: OptionParams, opt: OptionType) -> f64 {
        let h = 0.0001; // bump volatility by 1bp
        let seed = self.seed;
        let up = self.price_with_seed(OptionParams { sigma: p.sigma + h, ..p }, opt, seed);
        let down = self.price_with_seed(OptionParams { sigma: p.sigma - h, ..p }, opt, seed);
        (up - down) / (2.0 * h)
    }

    fn rho(&self, p: OptionParams, opt: OptionType) -> f64 {
        let h = 0.0001; // bump rate by 1bp
        let seed = self.seed;
        let up = self.price_with_seed(OptionParams { r: p.r + h, ..p }, opt, seed);
        let down = self.price_with_seed(OptionParams { r: p.r - h, ..p }, opt, seed);
        (up - down) / (2.0 * h)
    }

    fn theta(&self, p: OptionParams, opt: OptionType) -> f64 {
        let h = 1.0 / 365.0; // 1 day
        if p.t < h {
            return 0.0; // no time left
        }
        let seed = self.seed;
        let up = self.price_with_seed(OptionParams { t: p.t - h, ..p }, opt, seed);
        let mid = self.price_with_seed(p, opt, seed);
        (up - mid) / h
    }
}
