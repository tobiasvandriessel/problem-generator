use std::path::PathBuf;
use std::fs;

use problem_generator::{self, problem::problem_generation::{run_opt, ProblemOpt, ProblemCommand::ConfigurationFolder}};

//DEV: when changes are made to the reproducability in any way, regenerate the problem output to add here, by running:
// ./target/release/problem_generator -s 2398 configuration_folder ./data/tiny_test
#[test]
fn deceptive_trap_generated() {
    let problem_command = ConfigurationFolder {
        folder_paths: vec![PathBuf::from("./data/tiny_test")],
        number_of_problems_to_generate: 1,
    };

    let problem_opt = ProblemOpt {
        problem_command: problem_command,
        seed: Some(2398)
    };

    run_opt(problem_opt).unwrap();

    let problem_actual_output = fs::read_to_string("./data/tiny_test/problems/deceptive_trap/deceptive-trap_5_3_1_2_0.txt")
        .expect("Could not read problem result from problem generation");
    
        let problem_expected_output = 
"5 3 1 2
4.8
1
00010111100
9 8 10
9 5 4
9 7 0
4 6 1
9 3 2
";

    assert_eq!(problem_actual_output, problem_expected_output);

    let codomain_expected_output = 
"deceptive-trap
5 3 1 2
0.30000000000000004
0.6000000000000001
1
0.30000000000000004
0.6000000000000001
0.9
0.30000000000000004
0.6000000000000001
0.6000000000000001
0.30000000000000004
0.9
0.6000000000000001
0.30000000000000004
1
0.6000000000000001
0.30000000000000004
0.6000000000000001
0.30000000000000004
0.9
0.6000000000000001
0.30000000000000004
1
0.6000000000000001
0.30000000000000004
0.30000000000000004
0.6000000000000001
1
0.30000000000000004
0.6000000000000001
0.9
0.30000000000000004
0.6000000000000001
0.30000000000000004
0.6000000000000001
1
0.30000000000000004
0.6000000000000001
0.9
0.30000000000000004
0.6000000000000001
";


    let codomain_actual_output = fs::read_to_string("./data/tiny_test/codomain_files/deceptive_trap/deceptive-trap_5_3_1_2_0.txt")
        .expect("Could not read codomain result from problem generation");

    assert_eq!(codomain_actual_output, codomain_expected_output);
}