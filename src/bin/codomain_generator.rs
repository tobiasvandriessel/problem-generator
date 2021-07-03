use problem_generator::problem::codomain::CodomainOpt;
use structopt::StructOpt;

use std::process;

fn main() {
    let codomain_opt = CodomainOpt::from_args();
    println!("{:?}", codomain_opt);

    problem_generator::problem::codomain::run_opt(codomain_opt).unwrap_or_else(|err| {
        eprintln!("Problem encountered while generating the codomain: {}", err);
        process::exit(1);
    });
}
