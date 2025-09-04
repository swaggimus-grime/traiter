use statrs::distribution::{Continuous, ContinuousCDF, Normal};
use crate::option::{OptionParams, OptionType};
use crate::pricers::Pricer;

pub struct BlackScholes;

impl BlackScholes {
    fn d1d2(&self, p: OptionParams) -> (f64, f64) {
        let sqrt_t = p.t.sqrt();
        let sigma_sqrt_t = p.sigma * sqrt_t;
        let d1 = ((p.s0/p.k).ln() + (0.5*p.sigma*p.sigma)*p.t) / sigma_sqrt_t;
        let d2 = d1 - sigma_sqrt_t;
        (d1, d2)
    }
}

impl Pricer for BlackScholes {
    fn price(&self, p: OptionParams, opt: OptionType) -> f64 {
        p.validate().expect("invalid params");

        if p.t == 0.0 {
            return match opt {
                OptionType::Call => (p.s0 - p.k).max(0.0),
                OptionType::Put => (p.k - p.s0).max(0.0),
            };
        }

        let sqrt_t = p.t.sqrt();
        let sigma_sqrt_t = p.sigma * sqrt_t;
        let d1 = ((p.s0/p.k).ln() + (0.5*p.sigma*p.sigma)*p.t) / sigma_sqrt_t;
        let d2 = d1 - sigma_sqrt_t;

        let n = Normal::new(0.0,1.0).unwrap();
        let nd1 = n.cdf(d1);
        let nd2 = n.cdf(d2);

        match opt {
            OptionType::Call => p.s0 * nd1 - p.k * (-p.r*p.t).exp() * nd2,
            OptionType::Put => p.k * (-p.r*p.t).exp() * (1.0 - nd2) - p.s0 * (1.0 - nd1),
        }
    }

    fn delta(&self, p: OptionParams, opt: OptionType) -> f64 {
        let (d1, _) = self.d1d2(p);
        let norm = Normal::new(0.0, 1.0).unwrap();
        match opt {
            OptionType::Call => norm.cdf(d1),
            OptionType::Put  => norm.cdf(d1) - 1.0,
        }
    }

    fn gamma(&self, p: OptionParams, opt: OptionType) -> f64 {
        let (d1, _) = self.d1d2(p);
        let norm = Normal::new(0.0, 1.0).unwrap();
        norm.pdf(d1) / (p.s0 * p.sigma * p.t.sqrt())
    }

    fn vega(&self, p: OptionParams, opt: OptionType) -> f64 {
        let (d1, _) = self.d1d2(p);
        let norm = Normal::new(0.0, 1.0).unwrap();
        p.s0 * norm.pdf(d1) * p.t.sqrt()
    }

    fn theta(&self, p: OptionParams, opt: OptionType) -> f64 {
        let (d1, d2) = self.d1d2(p);
        let norm = Normal::new(0.0, 1.0).unwrap();
        let first_term = -(p.s0 * norm.pdf(d1) * p.sigma) / (2.0 * p.t.sqrt());
        match opt {
            OptionType::Call => first_term - p.r * p.k * (-p.r * p.t).exp() * norm.cdf(d2),
            OptionType::Put  => first_term + p.r * p.k * (-p.r * p.t).exp() * norm.cdf(-d2),
        }
    }

    fn rho(&self, p: OptionParams, opt: OptionType) -> f64 {
        let (_, d2) = self.d1d2(p);
        let norm = Normal::new(0.0, 1.0).unwrap();
        match opt {
            OptionType::Call => p.k * p.t * (-p.r * p.t).exp() * norm.cdf(d2),
            OptionType::Put  => -p.k * p.t * (-p.r * p.t).exp() * norm.cdf(-d2),
        }
    }
}
