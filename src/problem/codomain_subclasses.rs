/*!
Module with all implemented codomain subclasses that can be generated with the codomain generator.
*/

use structopt::StructOpt;

use super::clique_tree::{get_possible_substrings, InputParameters};

use std::fmt;

use rand::distributions::Uniform;
use rand::prelude::*;
use rand_chacha::ChaChaRng;

///Enum to represent various codomain classes
#[repr(C)]
#[derive(Debug, StructOpt, PartialOrd, PartialEq, Clone)]
#[structopt()]
pub enum CodomainFunction {
    Random,
    Trap,
    DeceptiveTrap,
    #[structopt(name = "nk-q")]
    NKq {
        q: u32,
    },
    #[structopt(name = "nk-p")]
    NKp {
        p: f64,
    },
    ///Combination of random and deceptive trap, where every clique/subfunction has probability p_deceptive to be deceptive and (1 - p_deceptive) to be random
    RandomDeceptiveTrap {
        p_deceptive: f64,
    },
    Unknown,
}

impl CodomainFunction {
    //Get string representation of CodomainFunction, for use with filenames
    pub fn to_io_string(&self) -> String {
        match &self {
            CodomainFunction::Random => "random".to_owned(),
            CodomainFunction::Trap => "trap".to_owned(),
            CodomainFunction::DeceptiveTrap => "deceptive-trap".to_owned(),
            CodomainFunction::NKq { q } => format!("nk-q-{}", q),
            CodomainFunction::NKp { p } => format!("nk-p-{}", p),
            CodomainFunction::RandomDeceptiveTrap { p_deceptive } => {
                format!("random-deceptive-trap-{}", p_deceptive)
            }
            CodomainFunction::Unknown => "unknown".to_owned(),
        }
    }
}
impl fmt::Display for CodomainFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            CodomainFunction::Random => write!(f, "random"),
            CodomainFunction::Trap => write!(f, "trap"),
            CodomainFunction::DeceptiveTrap => write!(f, "deceptive-trap"),
            CodomainFunction::NKq { q } => write!(f, "nk-q {}", q),
            CodomainFunction::NKp { p } => write!(f, "nk-p {}", p),
            CodomainFunction::RandomDeceptiveTrap {
                p_deceptive: p_random,
            } => {
                write!(f, "random-deceptive-trap {}", p_random)
            }
            CodomainFunction::Unknown => write!(f, "unknown"),
        }
    }
}

///Generate random codomain values
pub fn generate_random(input_parameters: &InputParameters, rng: &mut ChaChaRng) -> Vec<Vec<f64>> {
    let die = Uniform::from(0.0..1.0);

    let m = input_parameters.m;
    let k = input_parameters.k;

    //Ensure k is smaller than 32, as otherwise the bit shift goes out of bounds on 32-bit machines
    assert!(k < 32);

    let mut codomain_tree = Vec::with_capacity(m as usize);

    for _ in 0..m {
        let mut codomain_clique = Vec::with_capacity((1 << k) as usize);
        for _ in 0..(1 << k) {
            codomain_clique.push(die.sample(rng));
        }
        codomain_tree.push(codomain_clique);
    }

    codomain_tree
}

///Generate general deceptive trap values:
/// For each subfunction, the local deceptor / local deceptive attractor is a random bit string of length k
///  and the local optimum is the inverse of that random bit string.
/// The codomain values for each bit string other than these two is defined by their hamming distance to the local deceptive attractor:
///  0.9 - d * 0.9/k , where d is the hamming distance to the local deceptive attractor.
/// The codomain value for the local optimum is 1.0
pub fn generate_trap_general(input_parameters: &InputParameters, rng: &mut ChaChaRng) -> Vec<Vec<f64>> {
    let m = input_parameters.m;
    let k = input_parameters.k;

    //Ensure k is smaller than 32, as otherwise the bit shift goes out of bounds on 32-bit machines
    assert!(k < 32);

    let possible_clique_substrings = get_possible_substrings(k);

    let mut codomain = Vec::with_capacity(m as usize);
    for _i in 0..m {
        let local_deceptor = get_random_solution(k, rng);

        let mut codomain_clique = Vec::with_capacity(1 << k);
        for j in 0..(1 << k) {
            // d
            let distance_to_deceptor =
                get_hamming_distance_to_solution(&local_deceptor, &possible_clique_substrings[j]);
            let value = if distance_to_deceptor == k {
                //if local optimum
                1.0
            } else {
                //otherwise it's the local deceptive attractor or any other bit string
                0.9 - distance_to_deceptor as f64 * (0.9 / k as f64)
            };
            codomain_clique.push(value);
        }
        codomain.push(codomain_clique);
    }

    codomain
}

///Generate the codomain for the combination of random and deceptive trap codomain functions:
/// With probability p_deceptive, each clique/subfunction is a deceptive trap function,
///  and with probability (1 - p_deceptive) each clique/subfunction is a random function.
pub fn generate_random_trap(input_parameters: &InputParameters, p_deceptive: f64, rng: &mut ChaChaRng) -> Vec<Vec<f64>> {
    let die = Uniform::from(0.0..1.0);

    let m = input_parameters.m;
    let k = input_parameters.k;

    //Ensure k is smaller than 32, as otherwise the bit shift goes out of bounds on 32-bit machines
    assert!(k < 32);

    let possible_clique_substrings = get_possible_substrings(k);
    let mut codomain_tree = Vec::with_capacity(m as usize);

    for _ in 0..m {
        let mut codomain_clique = Vec::with_capacity(1 << k);

        if die.sample(rng) > p_deceptive {
            //Random
            for _ in 0..(1 << k) {
                codomain_clique.push(die.sample(rng));
            }
        } else {
            //Deceptive trap
            let local_deceptor = get_random_solution(k, rng);

            for j in 0..(1 << k) {
                let distance_to_deceptor = get_hamming_distance_to_solution(
                    &local_deceptor,
                    &possible_clique_substrings[j],
                );
                let value = if distance_to_deceptor == k {
                    1.0
                } else {
                    0.9 - distance_to_deceptor as f64 * (0.9 / k as f64)
                };
                codomain_clique.push(value);
            }
        }

        codomain_tree.push(codomain_clique);
    }

    codomain_tree
}

///Get the hamming distance to a solution, by counting the number of unequal bits in the bit strings
fn get_hamming_distance_to_solution(target_solution: &[u32], solution: &[u32]) -> u32 {
    assert_eq!(target_solution.len(), solution.len());

    let mut distance = 0;
    for i in 0..solution.len() {
        if target_solution[i] != solution[i] {
            distance += 1;
        }
    }
    distance
}

///Constrcut a trap codomain, with the definition as in <http://www.cs.uu.nl/docs/vakken/ea/pdf/Prac1.pdf>
pub fn generate_trap(input_parameters: &InputParameters, d: f64) -> Vec<Vec<f64>> {
    let m = input_parameters.m;
    let k = input_parameters.k;

    //Ensure k is smaller than 32, as otherwise the bit shift goes out of bounds
    assert!(k < 32);

    let multiplication_factor = ((k as f64) - d) / ((k - 1) as f64);

    let mut codomain_clique = Vec::with_capacity(1 << k);
    for i in 0..(1 << k) {
        if count_ones(k, i) == k {
            codomain_clique.push(k as f64);
        } else {
            codomain_clique
                .push(((k as f64) - d - multiplication_factor * (count_ones(k, i) as f64)) as f64);
        }
    }

    (0..m).map(|_| codomain_clique.clone()).collect()
}

///Generate NKq codomain values
///The q value indicates the highest integer value possible, every codomain value is generated randomly between 0..q(exclusive)
pub fn generate_nk_q(input_parameters: &InputParameters, q: u32, rng: &mut ChaChaRng) -> Vec<Vec<f64>> {
    let m = input_parameters.m;
    let k = input_parameters.k;

    let die = Uniform::from(0..q);

    let mut codomain = Vec::with_capacity(m as usize);
    for _ in 0..m {
        let codomain_clique: Vec<f64> = (0..(1 << k))
            .map(|_| die.sample(rng) as f64 / (q - 1) as f64)
            .collect();
        codomain.push(codomain_clique);
    }
    codomain
}

///Generate NKp codomain values
///The p value indicated the percentage of codomain values to be 0, per clique
pub fn generate_nk_p(input_parameters: &InputParameters, p: f64, rng: &mut ChaChaRng) -> Vec<Vec<f64>> {
    let m = input_parameters.m;
    let k = input_parameters.k;

    let num_zeroes = (p * (1 << k) as f64).round() as u32;

    let die = Uniform::from(0.0..1.0);

    let mut codomain_clique_indices: Vec<u32> = (0..(1 << k)).collect();
    let mut codomain = Vec::with_capacity(m as usize);

    for _ in 0..m {
        let mut codomain_clique = Vec::with_capacity(k as usize);
        codomain_clique_indices.shuffle(rng);

        let no_contribution_indices: Vec<&u32> = codomain_clique_indices
            .iter()
            .take(num_zeroes as usize)
            .collect();

        for i in 0..(1 << k) {
            if no_contribution_indices.contains(&&i) {
                codomain_clique.push(0.0);
            } else {
                codomain_clique.push(die.sample(rng));
            }
        }
        codomain.push(codomain_clique);
    }

    codomain
}

///Count the number of ones in the bit string represented by and as the index
fn count_ones(k: u32, index: u32) -> u32 {
    //Bit shift every element to the first index and then AND it with 1 to be able to add the number 1 to the sum,
    // for every 1 in the bit string.
    let mut sum = 0;
    for j in 0..k {
        sum += (index >> j) & 1;
    }
    sum
}

///Get a random solution, given the problem size
fn get_random_solution(problem_size: u32, rng: &mut ChaChaRng) -> Vec<u32> {
    let die = Uniform::from(0..2);
    (0..problem_size).map(|_| die.sample(rng)).collect()
}
