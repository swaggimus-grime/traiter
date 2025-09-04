use crate::option::{OptionParams, OptionType};

mod monte_carlo;
mod black_scholes;

pub use monte_carlo::*;
pub use black_scholes::*;

pub trait Pricer {
    fn price(&self, p: OptionParams, opt: OptionType) -> f64;
    fn price_parallel(&self, p: OptionParams, opt: OptionType) -> f64 {
        unimplemented!()
    }
    fn delta(&self, p: OptionParams, opt: OptionType) -> f64;
    fn gamma(&self, p: OptionParams, opt: OptionType) -> f64;
    fn vega(&self, p: OptionParams, opt: OptionType) -> f64;
    fn theta(&self, p: OptionParams, opt: OptionType) -> f64;
    fn rho(&self, p: OptionParams, opt: OptionType) -> f64;
}

