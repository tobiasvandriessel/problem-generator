use structopt::StructOpt;
use problem_generator::problem::problem::ProblemOpt;

use std::process;

fn main() {
    let problem_opt = ProblemOpt::from_args();
    println!("{:?}", problem_opt);

    problem_generator::problem::problem::run_opt(problem_opt).unwrap_or_else(|err| {
        eprintln!("Problem encountered while generating the problem: {}", err);
        process::exit(1);
    });
}