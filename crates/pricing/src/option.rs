#[derive(Clone, Copy, Debug)]
pub enum OptionType { Call, Put }

#[derive(Clone, Copy, Debug)]
pub struct OptionParams {
    pub s0: f64,
    pub k: f64,
    pub r: f64,
    pub sigma: f64,
    pub t: f64,
}

impl OptionParams {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.s0 <= 0.0 || self.k <= 0.0 { return Err("S0 and K must be > 0"); }
        if self.t < 0.0 { return Err("T must be >= 0"); }
        if self.sigma < 0.0 { return Err("sigma must be >= 0"); }
        Ok(())
    }
}

pub fn payoff(opt: OptionType, st: f64, k: f64) -> f64 {
    match opt {
        OptionType::Call => (st - k).max(0.0),
        OptionType::Put => (k - st).max(0.0),
    }
}
