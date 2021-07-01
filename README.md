<!-- omit in toc -->
# Problem Generator - A TD Mk Landscape benchmark generator

<!-- TODO: abstract, paper, source code, install script, executable. -->
<!-- example problem instance text file, with k = 5, o = 2, M = 10 -->

This repository contains source code for the problem generator introduced in the workshop paper `Benchmark generator for TD Mk Landscapes' @ GECCO '21 Analysing Algorithmic Behaviour of Optimisation Heuristics Workshop, by Tobias van Driessel and Dirk Thierens. LINK TODO. The problem generator is a binary/library to generate TD Mk Landscapes (with fixed clique/subfunction size k, overlapping variables between cliques/subfunctions o, and number of branches in the clique tree b values). 
Main functionality:
* generation of problems & calculation of these problems's global optimum (or optima)
* generation of some input codomain files for the problem generation

This inclusion of codomain files generation should make it easy to generate a TD Mk Landscape problem from scratch and benchmark an algorithm with it.


## Workshop paper

### Abstract 

We introduce a publicly available benchmark generator for Tree Decomposition (TD) Mk Landscapes. TD Mk Landscapes were introduced by Whitley et al. [\[1\]](#references) to get rid of unnecessary restrictions of Adjacent NK Landscapes while still allowing for the calculation of the global optimum in polynomial time. This makes TD Mk Landscapes more lenient while still being as convenient as Adjacent NK Landscapes. Together, these properties make it very suitable for benchmarking blackbox algorithms. Whitley et al., however, introduced a construction algorithm that only constructs Adjacent NK Landscapes. Recently, Thierens et al. [\[2\]](#references) introduced an algorithm, CliqueTreeMk, to construct any TD Mk Landscape and find its optimum. In this work, we introduce CliqueTreeMk in more detail, implement it for public use, and show some results for LT-GOMEA on an example TD Mk Landscape problem. The results show that deceptive trap problems with higher overlap do not necessarily decrease performance and effectiveness for LT-GOMEA.



<!-- omit in toc -->
## Table of Contents
- [Workshop paper](#workshop-paper)
  - [Abstract](#abstract)
- [Quick Start](#quick-start)
  - [Binary](#binary)
  - [Library](#library)
- [Installation](#installation)
  - [Binary](#binary-1)
- [Usage](#usage)
  - [Problem Generation](#problem-generation)
    - [Configuration Input](#configuration-input)
    - [Codomain Input](#codomain-input)
    - [Codomain File Structure](#codomain-file-structure)
    - [Problem File Structure](#problem-file-structure)
- [Example](#example)
- [References](#references)

## Quick Start

### Binary 

The quickest way to start is by using ```cargo install problem_generator``` if you already have Rust installed or downloading the executable from the Release page if you don't.
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
problem_generator = "^0.1.0"
```

## Installation

### Binary

There are multiple options to choose from:
- Download executable:
	1. Download executable from the Release page
- Install using crates.io:  
	1. If you do not have Rust installed yet, run ```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh``` to install [Rustup](https://rustup.rs/) and the Rust programming language  
	2. Then run ```cargo install problem_generator``` to install the problem generator from [crates.io](https://crates.io/)  
- Compile  
	1. If you do not have Rust installed yet, run ```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh``` to install [Rustup](https://rustup.rs/) and the Rust programming language  
	2. Then checkout this repository ```git clone https://github.com/tobiasvandriessel/problem-generator && cd problem-generator```  
	3. and run ```cargo install --path .``` or ```cargo build --release```, 
	     depending on whether you want it installed or just built.  

## Usage

### Problem Generation

The problem generator can take as input a configuration folder, a codomain folder, a configuration file, or a codomain file. Here, we highlight how to use the generator with a configuration file and codomain file, and refer the reader to the documentation for the instructions on how to run the generator with multiple configuration files in a folder or multiple codomain files in a folder.

#### Configuration Input

We create a configuration file to generate deceptive trap problems with topology parameters in a range, in this case we use $M \in \{1, ..., 49\}$, $k = 5$, $o = 1$, $b = 1$: 
```
    M 1 50 
    k 5 6 
    o 1 2 
    b 1 2
    deceptive-trap
```

As options for the codomain we currently offer: *Random*, *Deceptive Trap*, *NKq*, *NKp*, and *Random Deceptive Trap* (a combination of the two). Here we have chosen the deceptive trap function.

Then we use the executable *problem\_generator* to generate the codomain files and the problems (25 for each configuration), and find the global optimum for each problem: 
``` 
    problem_generator configuration_file -n 25 FILE 
        CODOMAIN_OUT PROBLEM_OUT
```
where `CODOMAIN_OUT` and `PROBLEM_OUT` are the (existing) output codomain folder and output problem folder.

#### Codomain Input

Instead of generating the codomain and then generate a problem with this generated codomain, one can use an existing codomain file to create a TD Mk Landscape problem. The executable offers the following subcommand for this purpose: 
```
    problem_generator codomain_file CODOMAIN_FILE 
        PROBLEM_FILE_OUT 
```

#### Codomain File Structure
The input codomain files should have the following structure: 
```
    M K O B
    CODOMAIN_VALUE_1
    ...
    CODOMAIN_VALUE_LAST
```
where `M`, `K`, `O`, and `B` represent the to be inserted values of $M$, $k$, $o$ and $b$, and `CODOMAIN_VALUE_1` `...` `CODOMAIN_VALUE_LAST` represent the $M \cdot 2^k$ decimal codomain values, each on a new line. 

#### Problem File Structure

The output problem files have the following structure:
```
    M K O B
    GLOB_OPT_VAL
    NUM_GLOB_OPT
    GLOB_OPT_1
    ...
    GLOB_OPT_LAST
    CLIQUE_INDICES_1
    ...
    CLIQUE_INDICES_LAST
```
where `GLOB_OPT_VAL` represents the global optimum (optima) value, `NUM_GLOB_OPT` represents the number of global optima, `GLOB_OPT_1` `...` `GLOB_OPT_LAST` represent the global optima solutions, and `CLIQUE_INDICES_1` `...` `CLIQUE_INDICES_LAST` represent the problem variables in each clique. 

An example problem generated:

```
    2 5 1 1
    1.9
    2
    101000111
    010111000
    5 3 2 1 7
    1 0 6 4 8
```

## Example


## References 

\[1\] Darrell Whitley, Francisco Chicano, and Brian W Goldman. “Gray box optimization for Mk landscapes (NK landscapes and MAX-kSAT)”.

\[2\] Dirk Thierens, Tobias van Driessel. “A Benchmark Generator of Tree Decomposition Mk Landscapes”.