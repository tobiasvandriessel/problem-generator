/*!
Module for codomain generation, reading, and writing.
*/

use indicatif::ProgressIterator;
use rand_chacha::ChaChaRng;
use structopt::StructOpt;
use itertools::Itertools;

use super::io::get_output_folder_path_from_configuration_file;

use super::clique_tree::InputParameters;
use super::codomain_subclasses::*;
use super::configuration::{ConfigurationParameters, get_rng};

use std::fmt::Write as fmtWrite;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::path::PathBuf;
use std::{
    error::Error,
    fs::{self, remove_dir_all},
    str::Lines,
};

#[derive(StructOpt, Debug)]
#[structopt(
    name = "Codomain Generator",
    about = "Generate the codomain of a TD Mk Landscape using a file or cli arguments"
)]
pub struct CodomainOpt {
    #[structopt(subcommand)]
    pub codomain_command: CodomainCommand,
    #[structopt(short = "s", long = "seed")]
    pub seed: Option<u64>,
}

#[derive(StructOpt, Debug)]
pub enum CodomainCommand {
    /// Generate codomain values for configurations specified in a given file
    #[structopt(name = "folder")]
    Folder {
        ///File to read all the configurations from, for which codomains need to be generated
        #[structopt(parse(from_os_str))]
        folder_paths: Vec<PathBuf>,
    },
    /// Generate codomain values for configurations specified in a given file
    #[structopt(name = "file")]
    File {
        ///File to read all the configurations from, for which codomains need to be generated
        #[structopt(parse(from_os_str))]
        file_path: PathBuf,
    },
    /// Generate codomain values for the configuration defined by the cli arguments
    #[structopt(name = "instance")]
    Instance {
        /// The number of subfunctions
        m: u32,
        /// The size of the subfunctions
        k: u32,
        /// The number of overlapping bits between subfunctions
        o: u32,
        /// The branching factor
        b: u32,
        /// The output file
        #[structopt(name = "f", parse(from_os_str))]
        output_file_path: PathBuf,
        /// The subfunction to use for the codomain generation
        #[structopt(subcommand)]
        codomain_function: CodomainFunction,
    },
}

///Run codomain generator from command line options (structopt)
pub fn run_opt(codomain_opt: CodomainOpt) -> Result<(), Box<dyn Error>> {
    let mut rng = get_rng(codomain_opt.seed);
    match codomain_opt.codomain_command {
        CodomainCommand::Folder { folder_paths} => {
            for folder_path in folder_paths {
                handle_folder(folder_path, &mut rng)?;
            }
            Ok(())
        }
        CodomainCommand::File { file_path } => {
            handle_input_configuration_file(file_path, &mut rng)
        },
        CodomainCommand::Instance {
            m,
            k,
            o,
            b,
            output_file_path,
            codomain_function
        } => {
            let input_parameters = InputParameters::new_from_primitives(m, k, o, b);
            generate_and_write(&input_parameters, &codomain_function, &output_file_path, &mut rng)?;
            Ok(())
        }
    }
}

///Handle codomain generation for a folder: for every entry in it that is not a folder, pass the file to handle_input_file
fn handle_folder(folder_path: PathBuf, rng: &mut ChaChaRng) -> Result<(), Box<dyn Error>> {
    //First we remove all folders that are not named codomain_generation
    folder_path
        .read_dir()?
        .map(|file| file.unwrap())
        .filter(|file| {
            file.file_type().unwrap().is_dir() && file.file_name() != "codomain_generation"
        })
        .map(|file| remove_dir_all(file.path()))
        .collect::<Result<Vec<()>, std::io::Error>>()?;

    //Then we read every codomain generation file from the codomain_generation folder
    let mut codomain_generation_folder_path = folder_path;
    codomain_generation_folder_path.push("codomain_generation");
    let file_entries: Vec<PathBuf> = codomain_generation_folder_path
        .read_dir()?
        .map(|file| file.unwrap())
        .filter(|file| !file.file_type().unwrap().is_dir())
        .map(|file| file.path())
        .sorted()
        .collect();

    //And handle each of them
    file_entries.into_iter().progress().for_each(|path| {
        handle_input_configuration_file(path, rng).unwrap();
    });

    Ok(())
}

///Generate codomain from an input file (path), by reading the parameters from it,
/// getting the output directory path from the filename and generating the codomain 25 times for all input parameters.
fn handle_input_configuration_file(
    input_configuration_file_path: PathBuf,
    rng: &mut ChaChaRng
) -> Result<(), Box<dyn Error>> {
    let experiment_parameters = ConfigurationParameters::from_file(&input_configuration_file_path)?;
    let codomain_function = experiment_parameters.codomain_function.clone();
    let directory_path_buf = get_output_folder_path_from_configuration_file(
        &input_configuration_file_path,
        "codomain_files",
    )?;

    //Loop over all input parameters (using custom iterator)
    for input_parameters in experiment_parameters {
        //Generate 25 different codomain instances for each input parameter configuration
        for num in 0..25 {
            let mut output_file_path = directory_path_buf.clone();
            let output_file_name = format!(
                "{}_{}_{}_{}_{}_{}.txt",
                codomain_function.to_io_string(),
                input_parameters.m,
                input_parameters.k,
                input_parameters.o,
                input_parameters.b,
                num
            );

            output_file_path.push(output_file_name);
            //println!("constructed output file path: {:?}", output_file_path);

            generate_and_write(&input_parameters, &codomain_function, &output_file_path, rng)?;
        }
    }

    Ok(())
}

///Generate the codomain and write them to the file
fn generate_and_write(
    input_parameters: &InputParameters,
    codomain_function: &CodomainFunction,
    output_file_path: &Path,
    rng: &mut ChaChaRng
) -> Result<(), Box<dyn Error>> {
    write_codomain(
        input_parameters,
        codomain_function,
        output_file_path,
        &generate_codomain(input_parameters, codomain_function, rng),
    )?;
    Ok(())
}

///Generate the codomain, write them to the file, and return the codomain values
pub fn generate_write_return(
    input_parameters: &InputParameters,
    codomain_function: &CodomainFunction,
    output_file_path: &Path,
    rng: &mut ChaChaRng
) -> Result<Vec<Vec<f64>>, Box<dyn Error>> {
    let codomain = generate_codomain(input_parameters, codomain_function, rng);
    write_codomain(
        input_parameters,
        codomain_function,
        output_file_path,
        &codomain,
    )?;
    Ok(codomain)
}

///Generate the codomain, by calling the right generation function
pub fn generate_codomain(
    input_parameters: &InputParameters,
    codomain_function: &CodomainFunction,
    rng: &mut ChaChaRng
) -> Vec<Vec<f64>> {
    match codomain_function {
        CodomainFunction::Random => generate_random(input_parameters, rng),
        CodomainFunction::Trap => generate_trap(input_parameters, 2.5),
        CodomainFunction::DeceptiveTrap => generate_trap_general(input_parameters, rng), // generate_trap(input_parameters, 1.0),
        CodomainFunction::NKq { q } => generate_nk_q(input_parameters, *q, rng),
        CodomainFunction::NKp { p } => generate_nk_p(input_parameters, *p, rng),
        CodomainFunction::RandomDeceptiveTrap { p_deceptive } => {
            generate_random_trap(input_parameters, *p_deceptive, rng)
        }
        CodomainFunction::Unknown => panic!("We can't generate codomain for unknown codomain"),
    }
}

///Write the codomain to the passed file
fn write_codomain(
    input_parameters: &InputParameters,
    codomain_function: &CodomainFunction,
    file_path: &Path,
    codomain: &[Vec<f64>],
) -> Result<(), Box<dyn Error>> {
    let file = File::create(file_path)?;
    let mut buf_writer = BufWriter::new(file);
    let mut write_buffer = String::new();

    //Write the codomain function on the first line
    writeln!(write_buffer, "{}", codomain_function)?;
    buf_writer.write_all(write_buffer.as_bytes())?;
    write_buffer.clear();

    //Write the input parameters on the second line
    writeln!(
        write_buffer,
        "{} {} {} {}",
        input_parameters.m, input_parameters.k, input_parameters.o, input_parameters.b
    )?;
    buf_writer.write_all(write_buffer.as_bytes())?;
    write_buffer.clear();

    //Write all codomain values on the subsequent lines
    for clique in codomain {
        for value in clique {
            writeln!(write_buffer, "{}", value)?;
            buf_writer.write_all(write_buffer.as_bytes())?;
            write_buffer.clear();
        }
    }

    //Flush all data still in the buffer
    buf_writer.flush()?;

    Ok(())
}

///Get the codomain values from a file's content iterator
/// First skip a given number of lines and then read all the values
pub fn get_codomain_from_iterator(
    content_iterator: &mut Lines,
    skip_number_lines: u32,
    input_parameters: &InputParameters,
) -> Result<Vec<Vec<f64>>, Box<dyn Error>> {
    let mut content_iterator = content_iterator.skip(skip_number_lines as usize);
    let mut codomain = Vec::with_capacity(input_parameters.m as usize);
    for _i in 0..(input_parameters.m as usize) {
        let mut clique_codomain = Vec::with_capacity((1 << input_parameters.k) as usize);
        for _j in 0..(1 << input_parameters.k) {
            let fitness: f64 = content_iterator
                .next()
                .ok_or("Codomain file does not contain enough entries")?
                .parse()?;
            clique_codomain.push(fitness);
        }
        codomain.push(clique_codomain);
    }

    Ok(codomain)
}

///Read the codomain values from a file at the given path
pub fn read_codomain(
    input_parameters: &InputParameters,
    codomain_file: &Path,
    skip_number_lines: u32,
) -> Result<Vec<Vec<f64>>, Box<dyn Error>> {
    let contents = fs::read_to_string(codomain_file)?;
    //println!("contents of file: {}", contents);
    let mut content_iterator = contents.lines();
    get_codomain_from_iterator(&mut content_iterator, skip_number_lines, input_parameters)
}
