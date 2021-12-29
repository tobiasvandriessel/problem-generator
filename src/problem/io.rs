/*!
Module for functions related to reading and writing to files, mainly for reading stored clique trees
*/

use itertools::Itertools;
use rand_chacha::ChaChaRng;
use structopt::StructOpt;
// use itertools::Itertools;
use itertools::izip;

use std::{
    error::Error,
    fs::{self, remove_dir_all},
    path::{Path, PathBuf},
};

use super::clique_tree::{CliqueTree, InputParameters};
use super::codomain::read_codomain;
use super::codomain_subclasses::CodomainFunction;

/// Construct and return clique tree using input codomain file; use codomain and input parameters.
pub fn get_clique_tree_from_codomain_file(
    codomain_file_path: &Path,
    file_has_codomain_function: bool,
    rng: &mut ChaChaRng
) -> Result<CliqueTree, Box<dyn Error>> {
    let contents = fs::read_to_string(&codomain_file_path)?;
    let mut content_iterator = contents.lines();

    let codomain_function = if file_has_codomain_function {
        //Read codomain function
        let first_line = content_iterator
            .next()
            .ok_or("Input file does not contain enough entries")?;

        let mut iter_list = vec![" "];
        iter_list.extend(first_line.split(' '));
        CodomainFunction::from_iter(iter_list)
    } else {
        CodomainFunction::Unknown
    };

    //Read input parameters
    let input_parameters = InputParameters::from_line_iterator(&mut content_iterator)?;

    let skip_lines = if file_has_codomain_function { 2 } else { 1 };

    //Read codomain
    let codomain = read_codomain(&input_parameters, codomain_file_path, skip_lines)?;
    //print!("For file {:?} ", file_path);

    //Generate a clique tree that adheres to the given input parameters. The clique tree also calculates the global optimum.
    let clique_tree = CliqueTree::new(input_parameters, codomain_function, codomain, rng);

    //and return result
    Ok(clique_tree)
}

///Get the clique tree and path for each file in the passed codomain folder path
pub fn get_clique_trees_paths_from_codomain_folder(
    folder_path: &Path,
    files_have_codomain_function: bool,
    rng: &mut ChaChaRng
) -> Result<Vec<(CliqueTree, PathBuf)>, Box<dyn Error>> {
    Ok(folder_path
        .read_dir()?
        .map(|file| file.unwrap().path())
        .sorted()
        .map(|path| {
            (
                get_clique_tree_from_codomain_file(&path, files_have_codomain_function, rng).unwrap(),
                path,
            )
        })
        .collect())
}

///Get from a folder the triples configuration_parameters - problem_folder - codomain_folder
/// from the problem_generation, problems, and codomain_files folders.
/// Each file in problem_generation is coupled with the corresponding folder in 'problems' and 'codomain_files'
pub fn get_folders_file_triples(
    input_folder_path: &Path,
    remove_results_folder: bool,
) -> Result<Vec<(PathBuf, PathBuf, PathBuf)>, Box<dyn Error>> {
    //If we want to remove (previous) results, remove the results folder
    if remove_results_folder {
        let mut results_folder_path = PathBuf::from(input_folder_path);
        results_folder_path.push("results");
        let result = remove_dir_all(results_folder_path);

        if let Err(err) = result {
            if err.kind() != std::io::ErrorKind::NotFound {
                return Err(err).map_err(|_error| "could not remove results folder".into());
            }
        }
    }

    //Get the problem_generation, codomain_files, and problems folders.
    let mut problem_generation_folder = PathBuf::from(input_folder_path);
    problem_generation_folder.push("problem_generation");
    let mut codomain_files_folder = PathBuf::from(input_folder_path);
    codomain_files_folder.push("codomain_files");
    let mut problem_files_folder = PathBuf::from(input_folder_path);
    problem_files_folder.push("problems");

    //And read all files/folders inside these folders
    //Sort these, so that we can pass the files together without searching for the accompanying folder or file
    let file_entries: Vec<PathBuf> = problem_generation_folder
        .read_dir()?
        .map(|file| file.unwrap().path())
        .sorted()
        .collect();
    let codomain_folder_entries: Vec<PathBuf> = codomain_files_folder
        .read_dir()?
        .map(|file| file.unwrap().path())
        .sorted()
        .collect();
    let problem_folder_entries: Vec<PathBuf> = problem_files_folder
        .read_dir()?
        .map(|file| file.unwrap().path())
        .sorted()
        .collect();

    assert_eq!(file_entries.len(), codomain_folder_entries.len());
    assert_eq!(file_entries.len(), problem_folder_entries.len());

    //And couple the matching entries (matching is ensured by the sorting, as the same name is used for all three)
    Ok(izip!(
        file_entries,
        codomain_folder_entries,
        problem_folder_entries
    )
    .collect())
}

/// Get the output folder path for a given input configuration file
/// For example, passing "problem_generation/deceptive_trap_separated.txt"
///  and "results" , will create and return the folder "results/deceptive_trap_separated"
pub fn get_output_folder_path_from_configuration_file(
    input_configuration_file_path: &Path,
    output_directory_name: &str,
) -> Result<PathBuf, Box<dyn Error>> {
    let file_name = input_configuration_file_path.file_stem().ok_or(
        "could not get file stem of input configuration file while calculating output path",
    )?;

    let mut output_folder_path = PathBuf::from(
        input_configuration_file_path
            .parent()
            .ok_or("could not get parent")?
            .parent()
            .ok_or("could not get parent's parent")?,
    );
    output_folder_path.push(output_directory_name);
    output_folder_path.push(file_name);

    std::fs::create_dir_all(&output_folder_path)?;
    Ok(output_folder_path)
}
