/*!
Module for the 1) Clique Tree construction and global optimum calculation, 2) and the struct to contain solutions.
*/

use rand_chacha::ChaChaRng;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

use std::{error::Error, str::Lines};

use crate::problem::codomain_subclasses::CodomainFunction;
use crate::problem::problem_generation::Problem;

const FITNESS_EPSILON: f64 = 0.0000000001;

///Struct to contain the solution and its fitness, with the solution stored as a vector of u32 values (0 or 1) and the fitness as a f64 value
#[derive(Debug, Clone)]
pub struct SolutionFit {
    pub solution: Vec<u32>,
    pub fitness: f64,
}

///Struct to contain the input parameters of the TD Mk Landscape:
/// Number of cliques/subfunctions M,
/// size k of each clique/subfunction,
/// number of overlapping variables between cliques/subfunctions o,
/// number of branches in the clique tree / tree decomposition b
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct InputParameters {
    pub m: u32,
    pub k: u32,
    pub o: u32,
    pub b: u32,
}

impl InputParameters {
    pub fn new(args: &[String]) -> Result<InputParameters, &'static str> {
        if args.len() < 5 {
            return Err("not enough arguments");
        }

        let m = args[1]
            .clone()
            .parse::<u32>()
            .map_err(|_| "Could not parse M to u32")?;
        let k = args[2]
            .clone()
            .parse::<u32>()
            .map_err(|_| "Could not parse k to u32")?;
        let o = args[3]
            .clone()
            .parse::<u32>()
            .map_err(|_| "Could not parse o to u32")?;
        let b = args[4]
            .clone()
            .parse::<u32>()
            .map_err(|_| "Could not parse b to u32")?;

        //let codomain_file = args[5].clone();
        Ok(InputParameters { m, k, o, b })
    }

    pub fn new_from_primitives(m: u32, k: u32, o: u32, b: u32) -> InputParameters {
        InputParameters { m, k, o, b }
    }

    ///Get the input parameters from an iterator containing the line on which the parameters are listed
    pub fn from_line_iterator(
        content_iterator: &mut Lines,
    ) -> Result<InputParameters, Box<dyn Error>> {
        //Get the line that contains the parameters
        let line = content_iterator
            .next()
            .ok_or("Input file does not contain enough entries")?;
        //Split the line
        let parameters: Vec<&str> = line.split(' ').collect();
        if parameters.len() != 4 {
            return Err("not enough input parameters on first line of input file".into());
        }
        //And set the parameters
        let m: u32 = parameters[0].parse()?;
        let k: u32 = parameters[1].parse()?;
        let o: u32 = parameters[2].parse()?;
        let b: u32 = parameters[3].parse()?;

        Ok(InputParameters::new_from_primitives(m, k, o, b))
    }
}

#[derive(Debug)]
///The CliqueTree struct with properties input parameters, clique variable indices, the used codomain function, codomain values, global optimum strings and score
pub struct CliqueTree {
    pub input_parameters: InputParameters,
    pub codomain_function: CodomainFunction,
    pub cliques: Vec<Vec<u32>>,
    pub codomain_values: Vec<Vec<f64>>,
    pub glob_optima_strings: Vec<Vec<u32>>,
    pub glob_optima_score: f64,
}

impl CliqueTree {
    pub fn new(
        input_parameters: InputParameters,
        codomain_function: CodomainFunction,
        codomain_values: Vec<Vec<f64>>,
        rng: &mut ChaChaRng,
    ) -> CliqueTree {
        //Create a new clique tree (as its cliques and separators)
        let (cliques, separators) = CliqueTree::construct(&input_parameters, rng);

        //Then calculate the global optimum (optima) for the clique tree
        let global_opt_tuples = CliqueTree::calculate_global_optima(
            &input_parameters,
            &codomain_function,
            &codomain_values,
            &cliques,
            &separators,
        );

        let glob_optima_score = global_opt_tuples[0].1;
        let glob_optima_strings = global_opt_tuples.into_iter().map(|tuple| tuple.0).collect();

        // and return the resulting CliqueTree struct
        CliqueTree {
            input_parameters,
            codomain_function,
            cliques,
            codomain_values,
            glob_optima_strings,
            glob_optima_score,
        }
    }

    ///Construct the clique tree from the problem struct and codomain values
    pub fn construct_from_problem_codomain(problem: Problem, codomain: Vec<Vec<f64>>) -> Self {
        CliqueTree {
            input_parameters: problem.input_parameters,
            codomain_function: CodomainFunction::Unknown,
            cliques: problem.cliques,
            codomain_values: codomain,
            glob_optima_strings: problem.glob_optima_strings,
            glob_optima_score: problem.glob_optima_score,
        }
    }

    ///Calculate the global optimum for a separable problem
    fn calculate_global_optimum_separable(
        input_parameters: &InputParameters,
        codomain_values: &[Vec<f64>],
        cliques: &[Vec<u32>],
    ) -> Vec<(Vec<u32>, f64)> {
        //Set score to 0 and glob_optimum string to all zeroes.
        let mut glob_opt_score = 0.0;

        //Store the optimas per clique. The optima are stored as a number whose bit representation is the actual solution substring.
        let mut clique_optimas = Vec::with_capacity(input_parameters.m as usize);

        let mut number_global_optima_strings = 1;

        //Go over all 'cliques/subfunctions'
        for i in 0..input_parameters.m {
            //Set the current highest score for this subfunction to the string with all zeroes.
            let mut highest_score = codomain_values[i as usize][0];
            let mut highest_score_indices = vec![0];

            //Go over the rest of the possible permutations of the string.
            for j in 1..(1 << input_parameters.k) as usize {
                //And determine whether they have a higher score
                let score = codomain_values[i as usize][j as usize];
                if is_equal_fitness(score, highest_score) {
                    highest_score_indices.push(j as u32);
                } else if is_better_fitness(score, highest_score) {
                    highest_score = score;
                    highest_score_indices.clear();
                    highest_score_indices.push(j as u32);
                }
            }

            //Add the highest score to the global optimum score
            glob_opt_score += highest_score;

            //Calculate the number of global optima
            number_global_optima_strings *= highest_score_indices.len() as u32;
            //And push this clique's optima to the clique_optima list
            clique_optimas.push(highest_score_indices);
        }

        //Construct the global optima strings. First reserve space equal to the number of global optima, then add a first element.
        let mut result_optima_strings = Vec::with_capacity(number_global_optima_strings as usize);
        result_optima_strings.push(vec![0; (input_parameters.m * input_parameters.k) as usize]);

        //Construct the global optima
        CliqueTree::set_optimal_clique_substrings(
            input_parameters,
            cliques,
            &mut result_optima_strings,
            &clique_optimas,
            0,
        );

        //Return global optima strings and score
        result_optima_strings
            .into_iter()
            .map(|optimum| (optimum, glob_opt_score))
            .collect()
    }

    ///Construct the global optima, by inserting a clique's optimal substrings into the global optima strings and calling itself recursively for the next clique.
    ///When there are more than one optimal substrings for a clique, we clone the current global optima and then set all the values.
    fn set_optimal_clique_substrings(
        input_parameters: &InputParameters,
        cliques: &[Vec<u32>],
        result_optima_strings: &mut Vec<Vec<u32>>,
        clique_optimas: &[Vec<u32>],
        current_index: usize,
    ) {
        //If we handled all the cliques, exit.
        if current_index as u32 == input_parameters.m {
            return;
        }

        //Otherwise, first clone the current global optima strings
        let original_global_optima_length = result_optima_strings.len();
        //We want to clone (number_clique_optima - 1) times, as we already have one instance.
        for _ in 0..clique_optimas[current_index].len() - 1 {
            //clone the global optima
            for i in 0..original_global_optima_length {
                result_optima_strings.push(result_optima_strings[i].clone());
            }
        }

        //and then set the clique's optimal substrings's values in the global optima strings
        //Go over all the clique optima
        for (num, clique_optimum) in clique_optimas[current_index].iter().enumerate() {
            //And for each, we insert its values into the original global optima.
            for i in 0..original_global_optima_length {
                //Insert all its values
                for j in 0..input_parameters.k {
                    result_optima_strings[original_global_optima_length * num + i]
                        [cliques[current_index][j as usize] as usize] =
                        (clique_optimum >> (input_parameters.k - j - 1)) & 1;
                }
            }
        }

        //Call itself recursively to insert next clique's optimal values
        CliqueTree::set_optimal_clique_substrings(
            input_parameters,
            cliques,
            result_optima_strings,
            clique_optimas,
            current_index + 1,
        );
    }

    ///Calculate the global optima strings and fitnesses
    pub fn calculate_global_optima(
        input_parameters: &InputParameters,
        codomain_function: &CodomainFunction,
        codomain_values: &[Vec<f64>],
        cliques: &[Vec<u32>],
        separators: &[Vec<u32>],
    ) -> Vec<(Vec<u32>, f64)> {
        //If the problem is separable, we use a simple optimizer.
        if input_parameters.o == 0 {
            return CliqueTree::calculate_global_optimum_separable(
                input_parameters,
                codomain_values,
                cliques,
            );
        }

        //Capacity set to 2 right now, as I assume the number of global optima is low;
        // every time there are more than 2 C/S instances with the same score for a given seperator instance,
        // we will need to allocate memory, which is unwanted. Better be safe than sorry here.
        let size_per_separator_instance = if let CodomainFunction::NKq { q: _ } = codomain_function
        {
            1 << (input_parameters.k - input_parameters.o)
        } else {
            2
        };

        // [M][o] = [(best_string1, best_score), (best_string2, best_score)], so it saves the h_i by selecting
        //   the best strings with their score for each x_a and x_b value
        //possible TODO: Can't we store the index of the substring instead of the substring, i.e. u32 instead of Vec<u32>?
        //This should make sure that the inner vectors are initialized
        let mut best_scores: Vec<Vec<Vec<(Vec<u32>, f64)>>> =
            vec![
                vec![
                    Vec::with_capacity(size_per_separator_instance);
                    (1 << input_parameters.o) as usize
                ];
                input_parameters.m as usize
            ];

        //Determine number of levels to detect whether a clique has any children, and how to reach that child.
        //Also store the start indices for each level
        let mut sum = 0;
        let mut l = 0;
        let mut start_indices = Vec::new();
        while sum < input_parameters.m {
            start_indices.push(sum);
            sum += input_parameters.b.pow(l);
            l += 1;
        }

        //Set lowest level and its start index
        let start_index_lowest_level = sum - input_parameters.b.pow(l - 1);
        let lowest_level = l - 1;

        //Set current level and its start index
        let mut start_index_current_level = start_index_lowest_level;
        let mut current_level = lowest_level;

        //Calculate all possible substrings, so that we can easily store and retrieve the substrings for the given index.
        // This way, we don't need to use intermediate representations that use the substrings, but simply an index that points to the substring.
        let possible_clique_substrings = get_possible_substrings(input_parameters.k);
        let possible_separator_substrings = get_possible_substrings(input_parameters.o);
        let possible_clique_without_separator_substrings =
            get_possible_substrings(input_parameters.k - input_parameters.o);

        //Go over all nodes but the root, in reversed order.
        for i in (1..input_parameters.m).rev() {
            //Keep track of current level in the tree, and the current start index for that level
            if i < start_index_current_level {
                current_level -= 1;
                start_index_current_level = start_indices[current_level as usize];
            }

            //Iterate over all possible values for the separator, so that we can calculate h_i(x_a, x_b) for these values (of x_a and x_b).
            for j in 0..possible_separator_substrings.len() {
                //Keep track of highest score and the highest scoring Ci/Si values, for these Si values (j)
                //TODONE: replace this with another value as soon as we allow for multiple global optima. I can make these quite a bit bigger, as it's a small structure.
                let mut scores = Vec::with_capacity(1 << (input_parameters.k - input_parameters.o));
                let mut highest_score = 0.0;
                //Iterate over all possible values for Ci/Si. Store the score in the list if it has a higher score than the current highest score.
                for k in 0..possible_clique_without_separator_substrings.len() {
                    //Calculate f(x_p x_q x_r), which is given by the codomain values passed as input.
                    //I assume codomain is structured [M][k] = score
                    let mut score = codomain_values[i as usize]
                        [j * possible_clique_without_separator_substrings.len() + k]; //f
                                                                                      //Then, if it's a parent, add h_l for each child l.
                    if i < start_index_lowest_level {
                        let start_index_children = start_indices[(current_level + 1) as usize]
                            + input_parameters.b * (i - start_index_current_level);
                        for child_index in
                            start_index_children..(start_index_children + input_parameters.b)
                        {
                            //Make sure child exists!
                            if child_index >= input_parameters.m {
                                break;
                            }
                            //Maakt niet uit welke optie we kiezen toch? Want ze hebben allemaal dezelfde score en er hoeft verder nog niet gebrancht te worden,
                            // het enige dat belangrijk is, is dat we de hoogste score selecteren. Toch? Daarna kunnen we aangeven dat er meerdere globale optima zijn.
                            //Calculate the separator substring values for the current child, from the parent clique substring.
                            let separator_substring = get_child_separator_substring(
                                &cliques[i as usize],
                                &separators[child_index as usize],
                                &possible_clique_substrings
                                    [j * possible_clique_without_separator_substrings.len() + k],
                            );
                            //separators shouldn't break here, as we have now inserted a filler for 'separator 0', which doesn't exist,
                            // so everything should be aligned well.
                            //Add the h_l for this child l to the parent's score, by first transforming into an index variant (easier storage) and
                            // then retrieving the stored score of the child using the separator substring index.
                            let separator_substring_index_version =
                                transform_substring_vector_to_index(&separator_substring);
                            score += best_scores[child_index as usize]
                                [separator_substring_index_version as usize][0]
                                .1;
                            //h_child
                        }
                    }
                    //store temporarily highest score in scores
                    //This already allows for multiple highest scores
                    if !scores.is_empty() && is_better_fitness(score, highest_score) {
                        scores.clear();
                    }
                    if scores.is_empty() || is_better_or_equal_fitness(score, highest_score) {
                        //TODO: Here I could store k instead of the substring!
                        scores.push((
                            possible_clique_without_separator_substrings[k].clone(),
                            score,
                        ));
                        highest_score = score;
                    }
                }

                //store the highest score into h for that separator (i) and for these values of the separator(j)
                for tuple in scores.into_iter() {
                    //This shouldn't break anymore, as we should now have initialized the inner array (j as usize)
                    best_scores[i as usize][j as usize].push(tuple);
                }
            }
        }

        //Now we need to process the root to get the global optimum, which is just one more calculation,
        // but is different from the others, as it doesn't have a separator.

        //Store the scores again in a list
        let mut scores = Vec::with_capacity(1 << input_parameters.k);
        let mut highest_score = 0.0;

        //Iterate over all possible clique substrings / values for the root
        for c in 0..possible_clique_substrings.len() {
            //I assume codomain is structured [M][k] = score
            //Add f
            let mut score = codomain_values[0][c as usize]; //f

            //Add the h_l scores for each child l.
            let start_index_children = 1;
            for child_index in start_index_children..(start_index_children + input_parameters.b) {
                //Maakt niet uit welke optie we kiezen toch? Want ze hebben allemaal dezelfde score en er hoeft verder nog niet gebrancht te worden,
                // het enige dat belangrijk is, is dat we de hoogste score selecteren. Toch? Daarna kunnen we aangeven dat er meerdere globale optima zijn.

                //Make sure child exists!
                if child_index >= input_parameters.m {
                    break;
                }

                //Calculate the separator substring values for the current child, from the parent clique substring.
                let separator_substring = get_child_separator_substring(
                    &cliques[0],
                    &separators[child_index as usize],
                    &possible_clique_substrings[c],
                );
                //Add the h_l for this child l to the root clique's score, by first transforming into an index variant (easier storage) and
                // then retrieving the stored score of the child using the separator substring index.
                let separator_substring_index_version =
                    transform_substring_vector_to_index(&separator_substring);
                score += best_scores[child_index as usize]
                    [separator_substring_index_version as usize][0]
                    .1;
            }

            //store temporarily highest score in scores
            //This already allows for multiple highest scores
            if !scores.is_empty() && is_better_fitness(score, highest_score) {
                scores.clear();
            }
            if scores.is_empty() || is_better_or_equal_fitness(score, highest_score) {
                //TODO: Here I could store k instead of the substring!
                scores.push((possible_clique_substrings[c].clone(), score));
                highest_score = score;
            }
        }

        //store the highest score into h for that separator (i) and for these values (j)
        for tuple in &scores {
            debug!("Best clique0: {:?} with score {:?}", tuple.0, tuple.1);
        }

        //Now we want to construct the global optima string with the score we just calculated.
        // To construct the global optima, we just need to traverse the tree again, now starting from the top.
        //possible TODO: Count the number of multiple maximizing instances so that we can make
        //          an estimate of the number of global optima. I can just use a high number, as the structure is quite small and won't take much space

        let problem_size = (input_parameters.m - 1) * (input_parameters.k - input_parameters.o)
            + input_parameters.k;

        //initialize string that will store resulting global optimum string to zeroes
        let mut glob_opt_strings = Vec::with_capacity(40);

        //let mut glob_opt_string = vec![
        //    0;
        //    ((input_parameters.M - 1) * (input_parameters.k - input_parameters.o)
        //        + input_parameters.k) as usize
        //];
        //Create vector for global optimum substring for that clique, insert C0 already.
        //I couuuuld consider storing indices, but then I'd be constantly be translating these values from and to strings...
        //Only allocate space for the cliques that have a child, as it is temporary storage
        //let mut clique_opt_substrings =
        //vec![Vec::new(); (std::cmp::max(start_indices[lowest_level as usize], 1)) as usize];
        //clique_opt_substrings[0] = scores[0].0.clone();

        //Just take the first tuple of all the choices as the global optimum, ignore other possible global optima for now.
        //Set C0's global optimum substring values in the global optimum string
        for clique_opt in &scores {
            let mut new_glob_opt_string = vec![0; problem_size as usize];
            for index_in_clique in 0..input_parameters.k as usize {
                new_glob_opt_string[cliques[0][index_in_clique as usize] as usize] =
                    clique_opt.0[index_in_clique as usize];
            }
            glob_opt_strings.push(new_glob_opt_string);
        }
        //for index_in_clique in 0..input_parameters.k as usize {
        //    glob_opt_string[cliques[0][index_in_clique as usize] as usize] =
        //        scores[0].0[index_in_clique as usize];
        //}

        //Set level and start index to the first clique, as we're starting from the root and iterate to the end
        start_index_current_level = 0;
        current_level = 0;

        //Calculate the end of the loop
        let mut division = (input_parameters.m - 1) / input_parameters.b;
        if (input_parameters.m - 1) % input_parameters.b > 0 {
            division += 1;
        }

        //Go until latest node/clique with children
        for i in 0..division {
            if (current_level as usize) < (start_indices.len() - 1) {
                //Increase the current level in the tree when the considered index is at the next level's start index
                if i >= start_indices[(current_level + 1) as usize] {
                    current_level += 1;
                    start_index_current_level = start_indices[current_level as usize];
                }

                //Calculate the start_index for this clique's children
                let start_index_children = start_indices[(current_level + 1) as usize]
                    + input_parameters.b * (i - start_index_current_level);

                //Go over all its b children
                for j in 0..input_parameters.b {
                    //Break if the index of the child to consider goes out of the M range
                    if (i * input_parameters.b) + j >= input_parameters.m - 1 {
                        break;
                    }

                    //Get current considered child's index
                    let current_child_index = start_index_children + j;

                    //For all current global optimum strings, either fill in the only maximizing instance for this separator instance,
                    // or clone the global optimum string x times, for the x maximizing instances of this separator instance.
                    let glob_opt_strings_length = glob_opt_strings.len();
                    let mut glob_opt_strings_marked_deletion =
                        Vec::with_capacity(glob_opt_strings_length);
                    for k in 0..glob_opt_strings_length {
                        let glob_opt_string = &mut glob_opt_strings[k];

                        //Construct child's separator values using the global string values and the stored indices of the separator.
                        let separator_substring = get_separator_substring_from_string(
                            &separators[current_child_index as usize],
                            glob_opt_string,
                        );

                        //Get index for that substring, to index into h
                        let separator_substring_index_version =
                            transform_substring_vector_to_index(&separator_substring);

                        //For each maximizing instance for the given separator instance, clone the global string and
                        // set the maximizing instance values. These maximizing instance values are retrieved from h
                        //Get best tuple for that child's separator values from h:
                        let c_without_s_substrings: Vec<&Vec<u32>> = (&best_scores
                            [current_child_index as usize]
                            [separator_substring_index_version as usize])
                            .iter()
                            .map(|tuple| &tuple.0)
                            .collect();

                        //Remove the item currently in consideration? (check if loops don't break then)
                        // Then clone it a number of times equal to the number of maximizing instances for this separator,
                        //  and assign the bits from the maximizing instances.

                        //If there is just one maximizing instance for this seperator,
                        // then just insert the values for this instance into the current global optimum string
                        let number_maximizing_instances = c_without_s_substrings.len();
                        if number_maximizing_instances == 1 {
                            //Insert Ci/Si values into global optimum string
                            for index in 0..(input_parameters.k - input_parameters.o) {
                                glob_opt_string[cliques[current_child_index as usize]
                                    [(index + input_parameters.o) as usize]
                                    as usize] = c_without_s_substrings[0][index as usize];
                            }
                        } else {
                            //otherwise, clone the global optimum under consideration x times, where x is equal to the number of maximizing instances
                            // for this clique.

                            // make sure there are more than 0 maximizing instances
                            assert_ne!(
                                number_maximizing_instances, 0,
                                "there are 0 maximizing instances, which is impossible"
                            );

                            //direct naar glob_opt_strings pushen ipv eerst naar nieuwe array? -> Dit kan niet, doordat we nog een mutable borrow in scope hebben
                            //Clone the global optimum string under consideration and add to vector
                            let mut new_glob_opt_strings =
                                Vec::with_capacity(number_maximizing_instances);
                            for _l in 0..number_maximizing_instances {
                                new_glob_opt_strings.push(glob_opt_string.clone());
                            }

                            //For each maximizing instance, write the maximizing values to one of the cloned global optimum strings
                            for (num, maximizing_instance) in
                                c_without_s_substrings.iter().enumerate()
                            {
                                for index in 0..(input_parameters.k - input_parameters.o) {
                                    new_glob_opt_strings[num][cliques[current_child_index as usize]
                                        [(index + input_parameters.o) as usize]
                                        as usize] = maximizing_instance[index as usize];
                                }
                            }

                            //Append the newly created global optimum strings to the global optimum strings vector,
                            // and mark the global optimum string currenly under consideration as to be deleted.
                            glob_opt_strings.append(&mut new_glob_opt_strings);
                            glob_opt_strings_marked_deletion.push(k);
                        }
                    }

                    //Remove the global optimum strings that were marked as to be deleted,
                    // in reversed order, as we want to make sure that the indices correctly point to the strings to be deleted
                    for marked_index in glob_opt_strings_marked_deletion.into_iter().rev() {
                        glob_opt_strings.remove(marked_index);
                    }
                }
            }
        }

        for i in 1..input_parameters.m {
            for j in 0..(1 << input_parameters.o) {
                debug!(
                    "Best score for clique {:?} for index {:?}: {:?} with score {:?}",
                    i,
                    j,
                    best_scores[i as usize][j as usize][0].0,
                    best_scores[i as usize][j as usize][0].1
                );
            }
        }

        let glob_opt_score = scores.swap_remove(0).1;
        for glob_opt_string in &glob_opt_strings {
            debug!(
                "Glob opt string: {:?} and glob opt score: {:?}",
                glob_opt_string, glob_opt_score
            );
        }
        //Return the global optimum string and its fitness
        glob_opt_strings
            .into_iter()
            .map(|glob_opt_string| (glob_opt_string, glob_opt_score))
            .collect()
    }

    ///Construct the clique tree, using the input paramters and the codomain values. It returns a tuple (cliques, separators)
    pub fn construct(input_parameters: &InputParameters, rng: &mut ChaChaRng) -> (Vec<Vec<u32>>, Vec<Vec<u32>>) {
        let mut cliques: Vec<Vec<u32>> = Vec::with_capacity(input_parameters.m as usize);
        let mut separators: Vec<Vec<u32>> = Vec::with_capacity(input_parameters.m as usize);

        //Shuffle the variable indices, so that we don't get an easy tree.
        let mut indices: Vec<u32> = (0..((input_parameters.m - 1)
            * (input_parameters.k - input_parameters.o)
            + input_parameters.k))
            .collect();

        indices.shuffle(rng);
        debug!("{:?}", indices);

        //Initialize clique 0, C0, by  just taking the first k variable indices from the list.
        let mut clique0: Vec<u32> = Vec::with_capacity(input_parameters.k as usize);
        for i in 0..input_parameters.k {
            clique0.push(indices[i as usize]);
        }

        //Add C0 to the cliques list and add separator 0, S0, to the separator list. (S0 is dummy)
        cliques.push(clique0);
        separators.push(Vec::new()); //filler, there is no separator 0!

        //We set the number of currenlty constructed cliques to 1
        let mut count = 1;

        let b = if input_parameters.o == 0 {
            1
        } else {
            input_parameters.b
        };

        //We calculate the index of the first clique that should not get any children.
        let mut division = (input_parameters.m - 1) / b;
        //If a clique should construct at least one child, it is considered as well.
        if (input_parameters.m - 1) % b > 0 {
            division += 1;
        }

        //Dit kan nog geoptimaliseerd worden door die separator_count en variables_to_add ertussenuit te halen,
        // want daarna plaats ik het toch samen in een nieuwe vector...
        //Go over all cliques that will become a parent
        for i in 0..division {
            //Create b children, if possible
            for j in 0..b {
                //If b would be too many children, break as soon as we would create too many chldren.
                if (i * b) + j >= input_parameters.m - 1 {
                    break;
                }

                //Choose o random variable indices from Ci
                //Here, we first clone Ci, shuffle it, and push the first o variable indices to the separator.
                let mut clique_copy = cliques[i as usize].clone();
                clique_copy.shuffle(rng);

                let mut new_separator: Vec<u32> = Vec::with_capacity(input_parameters.o as usize);
                for k in 0..input_parameters.o {
                    new_separator.push(clique_copy[k as usize]);
                }

                //Set the start index of the k variable indices we will copy from the variable index list
                let start_index =
                    (count - 1) * (input_parameters.k - input_parameters.o) + input_parameters.k;
                //Copy the variable indices into a list
                let mut variables_to_add: Vec<u32> =
                    Vec::with_capacity((input_parameters.k - input_parameters.o) as usize);
                for k in 0..(input_parameters.k - input_parameters.o) {
                    variables_to_add.push(indices[(start_index + k) as usize]);
                }

                //Construct new clique for the child, by taking the o variables indices from the separator and
                // (k - o) variable indices from the variable index list
                let mut new_clique: Vec<u32> = Vec::with_capacity(input_parameters.o as usize);
                for k in 0..input_parameters.o {
                    new_clique.push(new_separator[k as usize]);
                }

                for k in 0..(input_parameters.k - input_parameters.o) {
                    new_clique.push(variables_to_add[k as usize]);
                }

                //Add the new clique and separator to the clique and separator list, increase the count of constructed cliques.
                cliques.push(new_clique);
                separators.push(new_separator);
                count += 1;
            }
        }

        debug!("{:?}", cliques);
        (cliques, separators)
    }

    ///Calculate the fitness of a passed solution using the knowledge that only one bit will be flipped,
    /// and given that the solution has **not** been mutated at the given index yet
    pub fn calculate_fitness_delta(
        &self,
        current_solutionfit: &SolutionFit,
        number_evaluations: &mut u32,
        index_mutation: u32,
    ) -> f64 {
        //First set the fitness to the current fitness
        let mut fitness = current_solutionfit.fitness;

        //Then loop over all the cliques
        for clique_index in 0..self.cliques.len() {
            let clique = &self.cliques[clique_index];
            if clique.contains(&index_mutation) {
                //And for each clique calculate the solution substring for this clique, as an index into an array of these substrings.
                let mut clique_substring_as_index = 0;
                //Create variable to conveniently store reference to the current clique in.
                let clique = &self.cliques[clique_index];

                //We will store the index in the clique of the bit that will be flipped
                let mut clique_mutation_index = 0;

                //Go over each variable index in the clique and for each one, take the bit value from the solution string and add it to the clique substring.
                for j in (0..clique.len()).rev() {
                    //If the solution index of the considered index is equal to the index of the mutated bit, we store the index (in this clique) for future use.
                    if clique[j] == index_mutation {
                        clique_mutation_index = j;
                    }

                    //As we would otherwise do, add all the bits from the solution to the clique's subsolution, to be evaluated hereafter
                    clique_substring_as_index +=
                        current_solutionfit.solution[clique[j] as usize] << (clique.len() - j - 1);
                }

                //Substract the fitness contribution of this clique, as this has been previously added to get to the current fitness.
                fitness -= self.codomain_values[clique_index][clique_substring_as_index as usize];

                //Now set the bit in the clique's subsolution to the value it would be after mutation.
                // It looks a bit involved, as we use u32 values.
                if current_solutionfit.solution[clique[clique_mutation_index] as usize] == 0 {
                    clique_substring_as_index += 1 << (clique.len() - clique_mutation_index - 1);
                } else {
                    clique_substring_as_index -= 1 << (clique.len() - clique_mutation_index - 1);
                }

                //Add the fitness contribution of this clique, taking into account the mutation.
                fitness += self.codomain_values[clique_index][clique_substring_as_index as usize];

                //Now we subtracted the old codomain value of this clique and have added the new value.
            }
        }

        *number_evaluations += 1;

        fitness
    }

    ///Calculate the fitnesss of a passed solution
    pub fn calculate_fitness_int(&self, solution: &[i32], number_evaluations: &mut u32) -> f64 {
        //First set the fitness to 0.0
        let mut fitness = 0.0;

        //Then loop over all the cliques
        for clique_index in 0..self.cliques.len() {
            //And for each clique calculate the solution substring for this clique, as an index into an array of these substrings.
            let mut clique_substring_as_index = 0;
            //Create variable to conveniently store reference to the current clique in.
            let clique = &self.cliques[clique_index];
            //Go over each variable index in the clique and for each one, take the bit value from the solution string and add it to the clique substring.
            for j in (0..clique.len()).rev() {
                clique_substring_as_index += solution[clique[j] as usize] << (clique.len() - j - 1);
            }

            //Add the fitness contribution of this clique
            fitness += self.codomain_values[clique_index][clique_substring_as_index as usize];
        }

        *number_evaluations += 1;

        fitness
    }

    ///Calculate the fitnesss of a passed solution
    pub fn calculate_fitness(&self, solution: &[u32], number_evaluations: &mut u32) -> f64 {
        //First set the fitness to 0.0
        let mut fitness = 0.0;

        //Then loop over all the cliques
        for clique_index in 0..self.cliques.len() {
            //And for each clique calculate the solution substring for this clique, as an index into an array of these substrings.
            let mut clique_substring_as_index = 0;
            //Create variable to conveniently store reference to the current clique in.
            let clique = &self.cliques[clique_index];
            //Go over each variable index in the clique and for each one, take the bit value from the solution string and add it to the clique substring.
            for j in (0..clique.len()).rev() {
                clique_substring_as_index += solution[clique[j] as usize] << (clique.len() - j - 1);
            }

            //Add the fitness contribution of this clique
            fitness += self.codomain_values[clique_index][clique_substring_as_index as usize];
        }

        *number_evaluations += 1;

        fitness
    }

    pub fn is_global_optimum(&self, solution_fit: &SolutionFit) -> bool {
        // if solution_fit.fitness != self.glob_optima_score
        //     && (self.glob_optima_score - solution_fit.fitness).abs() < 0.0000000001
        //     && (self.glob_optima_score - solution_fit.fitness).abs() >= FITNESS_EPSILON {
        //         println!("difference in fitness with global optimum was: {}", (self.glob_optima_score - solution_fit.fitness).abs() );
        //         panic!("global optimum found, but my current accepted range is too small: ");
        //     }
        solution_fit.fitness == self.glob_optima_score
            || ((self.glob_optima_score - solution_fit.fitness).abs() < FITNESS_EPSILON
                && self.glob_optima_strings.contains(&solution_fit.solution))
    }

    // pub fn is_global_optimum_solution_score(&self, solution: &[i32], score: f64) -> bool {
    //     // if solution_fit.fitness != self.glob_optima_score
    //     //     && (self.glob_optima_score - solution_fit.fitness).abs() < 0.0000000001
    //     //     && (self.glob_optima_score - solution_fit.fitness).abs() >= FITNESS_EPSILON {
    //     //         println!("difference in fitness with global optimum was: {}", (self.glob_optima_score - solution_fit.fitness).abs() );
    //     //         panic!("global optimum found, but my current accepted range is too small: ");
    //     //     }
    //     score == self.glob_optima_score
    //         || ((self.glob_optima_score - score).abs() < FITNESS_EPSILON
    //             && self.glob_optima_strings.iter().any(|x| x == solution as &[u32]))
    // }
}

pub fn is_better_solutionfit(solutionfit1: &SolutionFit, solutionfit2: &SolutionFit) -> bool {
    solutionfit1.fitness > solutionfit2.fitness
        && (solutionfit1.fitness - solutionfit2.fitness).abs() >= FITNESS_EPSILON
}

pub fn is_worse_solutionfit(solutionfit1: &SolutionFit, solutionfit2: &SolutionFit) -> bool {
    solutionfit1.fitness < solutionfit2.fitness
        && (solutionfit1.fitness - solutionfit2.fitness).abs() >= FITNESS_EPSILON
}

pub fn is_better_or_equal_solutionfit(
    solutionfit1: &SolutionFit,
    solutionfit2: &SolutionFit,
) -> bool {
    solutionfit1.fitness > solutionfit2.fitness || is_equal_solutionfit(solutionfit1, solutionfit2)
}

pub fn is_equal_solutionfit(solutionfit1: &SolutionFit, solutionfit2: &SolutionFit) -> bool {
    solutionfit1.fitness == solutionfit2.fitness
        || (solutionfit1.fitness - solutionfit2.fitness).abs() < FITNESS_EPSILON
            && solutionfit1.solution == solutionfit2.solution
}

pub fn is_better_fitness(fitness1: f64, fitness2: f64) -> bool {
    fitness1 > fitness2 && (fitness1 - fitness2).abs() >= FITNESS_EPSILON
}

pub fn is_worse_fitness(fitness1: f64, fitness2: f64) -> bool {
    fitness1 < fitness2 && (fitness1 - fitness2).abs() >= FITNESS_EPSILON
}

pub fn is_better_or_equal_fitness(fitness1: f64, fitness2: f64) -> bool {
    fitness1 > fitness2 || is_equal_fitness(fitness1, fitness2)
}

pub fn is_equal_fitness(fitness1: f64, fitness2: f64) -> bool {
    (fitness1 - fitness2).abs() < FITNESS_EPSILON
}

///Get an iterator for all possible substrings of certain length
pub fn get_possible_substrings_iter(length: u32) -> impl Iterator<Item = Vec<u32>> {
    assert!(length < 32);

    (0..(1 << length)).map(move |substring_as_index| {
        //bit shift to get vector representation of solution from bit string version
        (0..length)
            .rev()
            .map(|i| (substring_as_index >> i) & 1)
            .collect()
    })
}

/// Get all possible (sub)strings for a given length (bits)
pub fn get_possible_substrings(length: u32) -> Vec<Vec<u32>> {
    assert!(length < 32);

    (0..(1 << length))
        .map(|substring_as_index| {
            (0..length)
                .rev()
                .map(|i| (substring_as_index >> i) & 1)
                .collect()
        })
        .collect()
}

/// Get the separator substring for the child, by taking the string values from the parent clique
///   for the variable indices both in the parent clique and in the separator  
fn get_separator_substring_from_string(separator: &[u32], glob_string: &[u32]) -> Vec<u32> {
    //Get the substring values from the parent clique for the variable indices both in the parent clique and in the separator
    let mut separator_substring = Vec::with_capacity(separator.len());
    //For every variable index in the child separator, find the index of that variable index in the parent clique,
    // and copy that value into the separator substring.
    for &index in separator {
        separator_substring.push(glob_string[index as usize]);
    }
    separator_substring
}

/// Get the separator substring for the child, by taking the substring values from the parent clique
///   for the variable indices both in the parent clique and in the separator  
fn get_child_separator_substring(
    clique: &[u32],
    separator: &[u32],
    clique_substring: &[u32],
) -> Vec<u32> {
    //Get the substring values from the parent clique for the variable indices both in the parent clique and in the separator
    let mut separator_substring = Vec::with_capacity(separator.len());
    //For every variable index in the child separator, find the index of that variable index in the parent clique,
    // and copy that value into the separator substring.
    for &index in separator {
        let found_index = clique
            .iter()
            .position(|&x| x == index)
            .expect("index in separator not found in clique!");
        separator_substring.push(clique_substring[found_index]);
    }
    separator_substring
}

///Transform the passed substring into an index(bit value) that would point to that substring
pub fn transform_substring_vector_to_index(substring: &[u32]) -> u32 {
    let mut sum = 0;
    let mut current_bit_shift_amount = 0;
    //Calculate bit value using the input bit string
    for i in (0..substring.len()).rev() {
        sum += substring[i as usize] << current_bit_shift_amount;
        current_bit_shift_amount += 1;
    }
    sum
}
