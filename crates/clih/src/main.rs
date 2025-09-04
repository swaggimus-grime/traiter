use clap::Parser;
use pricing::{OptionParams, OptionType, BlackScholes, MonteCarlo, Pricer};

#[derive(Parser)]
struct Args {
    #[arg(long)] model: String,
    #[arg(long)] kind: String,
    #[arg(long)] s0: f64,
    #[arg(long)] k: f64,
    #[arg(long)] r: f64,
    #[arg(long)] sigma: f64,
    #[arg(long)] t: f64,
    #[arg(long, default_value_t=100000)] paths: usize,
    #[arg(long, default_value_t=42)] seed: u64,
}

fn main() {
    let args = Args::parse();
    let opt = match args.kind.as_str() {
        "call" => OptionType::Call,
        "put" => OptionType::Put,
        _ => panic!("kind must be 'call' or 'put'"),
    };
    let p = OptionParams { s0: args.s0, k: args.k, r: args.r, sigma: args.sigma, t: args.t };

    let price = match args.model.as_str() {
        "bs" => BlackScholes.price(p, opt),
        "mc" => MonteCarlo::new(args.paths, args.seed).price(p, opt),
        _ => panic!("model must be 'bs' or 'mc'"),
    };
    println!("Price: {:.6}", price);
}
