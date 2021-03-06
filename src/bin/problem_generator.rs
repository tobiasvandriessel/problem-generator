use problem_generator::problem::problem_generation::ProblemOpt;
use structopt::StructOpt;

use std::process;

fn main() {
    let problem_opt = ProblemOpt::from_args();
    println!("{:?}", problem_opt);

    problem_generator::problem::problem_generation::run_opt(problem_opt).unwrap_or_else(|err| {
        eprintln!("Problem encountered while generating the problem: {}", err);
        process::exit(1);
    });
}
