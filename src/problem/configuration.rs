/*!
Module for reading the configuration ranges (and iterating over it)
*/

use structopt::StructOpt;
use rand::SeedableRng;
use rand_chacha::ChaChaRng;

use std::{error::Error, path::Path};

use super::{clique_tree::InputParameters, codomain_subclasses::CodomainFunction};

///Struct to contain the configuration parameters, to conveniently iterate over
#[derive(Debug, Clone)]
pub struct ConfigurationParameters {
    pub m_begin: u32,
    pub m_end: u32,
    pub k_begin: u32,
    pub k_end: u32,
    pub o_begin: u32,
    pub o_end: u32,
    pub b_begin: u32,
    pub b_end: u32,
    pub codomain_function: CodomainFunction,
}

impl ConfigurationParameters {
    pub fn new(
        m_begin: u32,
        m_end: u32,
        k_begin: u32,
        k_end: u32,
        o_begin: u32,
        o_end: u32,
        b_begin: u32,
        b_end: u32,
        codomain_function: CodomainFunction,
    ) -> ConfigurationParameters {
        ConfigurationParameters {
            m_begin,
            m_end,
            k_begin,
            k_end,
            o_begin,
            o_end,
            b_begin,
            b_end,
            codomain_function,
        }
    }

    ///Read configuration parameters from a file
    pub fn from_file(input_file_path: &Path) -> Result<ConfigurationParameters, Box<dyn Error>> {
        let contents = std::fs::read_to_string(&input_file_path)?;
        let mut content_iterator = contents.lines();

        let mut split_line = content_iterator.next().unwrap().split(' ');
        let m_or_n = split_line.next().unwrap();
        // .skip(1);
        let m_or_n_begin: u32 = split_line.next().unwrap().parse()?;
        let m_or_n_end: u32 = split_line.next().unwrap().parse()?;

        let mut split_line = content_iterator.next().unwrap().split(' ').skip(1);
        let k_begin: u32 = split_line.next().unwrap().parse()?;
        let k_end: u32 = split_line.next().unwrap().parse()?;

        let mut split_line = content_iterator.next().unwrap().split(' ').skip(1);
        let o_begin: u32 = split_line.next().unwrap().parse()?;
        let o_end: i32 = split_line.next().unwrap().trim().parse()?;
        let o_end: u32 = o_end as u32;

        let mut split_line = content_iterator.next().unwrap().split(' ').skip(1);
        let b_begin: u32 = split_line.next().unwrap().parse()?;
        let b_end: u32 = split_line.next().unwrap().parse()?;

        let (m_begin, m_end) = if m_or_n == "M" {
            (m_or_n_begin, m_or_n_end)
        } else if m_or_n == "N" {
            if k_end - k_begin > 1 || o_end - o_begin > 1 {
                return Err("Can not use problem size in configuration when k and o are not one fixed value".into());
            }
            (
                get_m_for_min_problem_size(m_or_n_begin, k_begin, o_begin),
                get_m_for_max_problem_size(m_or_n_end, k_begin, o_begin),
            )
        } else {
            return Err("First letter in configuration not recognized; not M or N".into());
        };

        let codomain_functions_split_line: Vec<&str> =
            content_iterator.next().unwrap().split(',').collect();

        assert_eq!(codomain_functions_split_line.len(), 1);

        let codomain_function_string = String::from(codomain_functions_split_line[0]);
        let mut iter_list = vec![" "];
        iter_list.extend(codomain_function_string.split(' '));
        let codomain_function = CodomainFunction::from_iter(iter_list);

        Ok(ConfigurationParameters::new(
            m_begin,
            m_end,
            k_begin,
            k_end,
            o_begin,
            o_end,
            b_begin,
            b_end,
            codomain_function,
        ))
    }
}

///Get iterator from configuration parameters struct, for convenient iteration
impl IntoIterator for ConfigurationParameters {
    type Item = InputParameters;
    type IntoIter = ConfigurationParametersIterator;

    fn into_iter(self) -> Self::IntoIter {
        ConfigurationParametersIterator::from_configuration_parameters(&self)
    }
}

///Iterator to iterate over all possible experiment parameters
pub struct ConfigurationParametersIterator {
    pub m_begin: u32,
    pub m_end: u32,
    pub k_begin: u32,
    pub k_end: u32,
    pub o_begin: u32,
    pub o_end: u32,
    pub b_begin: u32,
    pub b_end: u32,
    pub codomain_function: CodomainFunction,

    pub current_parameters: InputParameters,
}

impl ConfigurationParametersIterator {
    pub fn new(
        m_begin: u32,
        m_end: u32,
        k_begin: u32,
        k_end: u32,
        o_begin: u32,
        o_end: u32,
        b_begin: u32,
        b_end: u32,
        codomain_function: CodomainFunction,
    ) -> ConfigurationParametersIterator {
        ConfigurationParametersIterator {
            m_begin,
            m_end,
            k_begin,
            k_end,
            o_begin,
            o_end,
            b_begin,
            b_end,
            codomain_function,
            current_parameters: InputParameters::new_from_primitives(0, 0, 0, 0),
        }
    }

    pub fn from_configuration_parameters(
        configuration_parameters: &ConfigurationParameters,
    ) -> ConfigurationParametersIterator {
        ConfigurationParametersIterator::new(
            configuration_parameters.m_begin,
            configuration_parameters.m_end,
            configuration_parameters.k_begin,
            configuration_parameters.k_end,
            configuration_parameters.o_begin,
            configuration_parameters.o_end,
            configuration_parameters.b_begin,
            configuration_parameters.b_end,
            configuration_parameters.codomain_function.clone(),
        )
    }
}

///Implement the Iterator trait for ConfigurationParameters; iterate over all possible configuration parameters
impl Iterator for ConfigurationParametersIterator {
    type Item = InputParameters;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_parameters.m == 0 {
            self.current_parameters = InputParameters::new_from_primitives(
                self.m_begin,
                self.k_begin,
                self.o_begin,
                self.b_begin,
            );
        } else if self.current_parameters.b < self.b_end - 1 {
            self.current_parameters.b += 1;
        } else if self.current_parameters.o < self.o_end - 1 {
            self.current_parameters.o += 1;
            self.current_parameters.b = self.b_begin;
        } else if self.current_parameters.k < self.k_end - 1 {
            self.current_parameters.k += 1;
            self.current_parameters.o = self.o_begin;
            self.current_parameters.b = self.b_begin;
        } else if self.current_parameters.m < self.m_end - 1 {
            self.current_parameters.m += 1;
            self.current_parameters.k = self.k_begin;
            self.current_parameters.o = self.o_begin;
            self.current_parameters.b = self.b_begin;
        } else {
            return None;
        }
        Some(self.current_parameters.clone())
    }
}

//min problem size (incl. )
fn get_m_for_min_problem_size(min_problem_size: u32, k: u32, o: u32) -> u32 {
    let a = (min_problem_size as i32 + (k - o) as i32 - k as i32) as f32 / (k - o) as f32;
    (a.ceil() as i32).max(1) as u32
}

//max problem size (excl. )
fn get_m_for_max_problem_size(max_problem_size: u32, k: u32, o: u32) -> u32 {
    let a = (max_problem_size + (k - o) - k) as f32 / (k - o) as f32;
    (a.ceil() as u32).max(2)
}

pub fn get_rng(seed: Option<u64>) -> ChaChaRng {
    match seed {
        Some(seed) => ChaChaRng::seed_from_u64(seed),
        None => ChaChaRng::from_entropy(),
    }
}