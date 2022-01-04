## Documentation Pages

- [Index](index.md)
  - [Subcommands](subcommands.md)
  - [File Structures](file_structures.md)
  - [Installation](installation.md)

# Problem Generator - A TD Mk Landscape benchmark generator

Main functionality:
* generation of problems & calculation of these problems's global optimum (or optima) 
* generation of some input codomain files for the problem generation

The problem generator has various [subcommands](subcommands.md) to switch between input modes: `configuration_folder`, `codomain_folder`, `configuration_file`, and `codomain_file`.
 
The file structure of the configuration, codomain, and (output) problem files is listed in [file_structures](file_structures.md).

The installation instructions are listed in [installation](installation.md).

## Quick Start

### Example output problem

An example output problem can be found in the [data/](https://github.com/tobiasvandriessel/problem-generator/tree/main/data/) folder, if one only wishes to use an example output problem.

### Binary 

The quickest way to start is by using ```cargo install problem_generator``` if you already have Rust installed or downloading the executable from the [Release](https://github.com/tobiasvandriessel/problem-generator/releases/latest) page if you don't.
Create a new root directory, where we will store the codomain and problem folders in, and create a new configuration file to generate some problems:
```
mkdir example/problem_generation -p
vim example/problem_generation/deceptive_trap_separated.txt
```

Copy in the following contents:
```
M 1 4
k 5 6
o 0 1
b 1 2
deceptive-trap
```

Then enter ```problem_generator configuration_folder example``` to generate 1 problem per configuration, which can be found in the `problems` folder. The accompanying codomain values can be found in the `codomain_files` folder.

### Library

To use problem_generator in your project, you can simply add problem_generator into your ```cargo.toml```: 

```toml
[dependencies]
problem_generator = "^0.3.0"
```

The library documentation can be found on [doc.rs](https://docs.rs/problem_generator/0.3.0).