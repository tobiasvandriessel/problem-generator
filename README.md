<!-- omit in toc -->
# Problem Generator - A TD Mk Landscape benchmark generator

<!-- TODO: abstract, paper, source code, install script, executable. -->
<!-- example problem instance text file, with k = 5, o = 2, M = 10 -->

This repository contains source code for the problem generator introduced in the workshop paper `Benchmark generator for TD Mk Landscapes' @ GECCO '21 Analysing Algorithmic Behaviour of Optimisation Heuristics Workshop, by Tobias van Driessel and Dirk Thierens : [https://dl.acm.org/doi/10.1145/3449726.3463177](https://dl.acm.org/doi/10.1145/3449726.3463177). This workshop paper was a part of my [master thesis research](https://www.dropbox.com/s/8gcxlszjxumogys/TDMkLandscape_poster.pdf?dl=0).

The problem generator is a binary/library to generate TD Mk Landscapes, which are useful for benchmarking Black Box Optimizers. 
<!-- (with fixed clique/subfunction size k, overlapping variables between cliques/subfunctions o, and number of branches in the clique tree b values)-->
Main functionality:
* generation of problems & calculation of these problems's global optimum (or optima)
* generation of some input codomain files for the problem generation

This inclusion of codomain files generation should make it easy to generate a TD Mk Landscape problem from scratch and benchmark an algorithm with it.


## Workshop paper Abstract

We introduce a publicly available benchmark generator for Tree Decomposition (TD) Mk Landscapes. TD Mk Landscapes were introduced by Whitley et al. [\[1\]](#references) to get rid of unnecessary restrictions of Adjacent NK Landscapes while still allowing for the calculation of the global optimum in polynomial time. This makes TD Mk Landscapes more lenient while still being as convenient as Adjacent NK Landscapes. Together, these properties make it very suitable for benchmarking blackbox algorithms. Whitley et al., however, introduced a construction algorithm that only constructs Adjacent NK Landscapes. Recently, Thierens et al. [\[2\]](#references) introduced an algorithm, CliqueTreeMk, to construct any TD Mk Landscape and find its optimum. In this work, we introduce CliqueTreeMk in more detail, implement it for public use, and show some results for LT-GOMEA on an example TD Mk Landscape problem. The results show that deceptive trap problems with higher overlap do not necessarily decrease performance and effectiveness for LT-GOMEA.



<!-- omit in toc -->
## Table of Contents
- [Workshop paper Abstract](#workshop-paper-abstract)
- [Quick Start](#quick-start)
	- [Example output problem](#example-output-problem)
	- [Binary](#binary)
	- [Library](#library)
		- [Rust](#rust)
		- [C++](#c)
- [Installation](#installation)
	- [Binary](#binary-1)
- [Usage/Documentation](#usagedocumentation)
- [References](#references)
- [License](#license)

## Quick Start

### Example output problem

An example output problem can be found in the [data/](https://github.com/tobiasvandriessel/problem-generator/tree/main/data/) folder, if one only wishes to use an example output problem.

### Binary 

The quickest way to start is by using ```cargo install problem_generator``` if you already have Rust installed or downloading the executable from the [Release](https://github.com/tobiasvandriessel/problem-generator/releases/latest) page if you don't. See [Installation](#installation) for more information.
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

#### Rust

To use problem_generator in your project, you can simply add problem_generator into your ```cargo.toml```: 

```toml
[dependencies]
problem_generator = "^0.3.0"
```

The library documentation can be found on [doc.rs](https://docs.rs/problem_generator/0.3.0).

#### C++

Current WIP is creating a wrapper in C++ for this library, for which most of the work is done and can be found in the [cpp-integration branch](https://github.com/tobiasvandriessel/problem-generator/tree/cpp_integration). This will be used by the [IOHprofiler/IOHexperimenter](https://github.com/IOHprofiler/IOHexperimenter) benchmark framework to integrate the TD Mk Landscape benchmark generator. Note that the C++ wrapper could be adjusted fairly easily into a C wrapper. 


## Installation

### Binary

There are multiple options to choose from:
- Download executable:
	1. Download executable from the [Release](https://github.com/tobiasvandriessel/problem-generator/releases/latest) page
- Install using crates.io:  
	1. If you do not have Rust installed yet, run ```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh``` to install [Rustup](https://rustup.rs/) and the Rust programming language  
	2. Then run ```cargo install problem_generator``` to install the problem generator from [crates.io](https://crates.io/)  
- Compile  
	1. If you do not have Rust installed yet, run ```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh``` to install [Rustup](https://rustup.rs/) and the Rust programming language  
	2. Then checkout this repository ```git clone https://github.com/tobiasvandriessel/problem-generator && cd problem-generator```  
	3. and run ```cargo install --path .``` or ```cargo build --release```, 
	     depending on whether you want it installed or just built.  

## Usage/Documentation

The [documentation](https://tobiasvandriessel.github.io/problem-generator/) explains the usage of the problem generator.



## References 

\[1\] Darrell Whitley, Francisco Chicano, and Brian W Goldman. “Gray box optimization for Mk landscapes (NK landscapes and MAX-kSAT)”.

\[2\] Dirk Thierens, Tobias van Driessel. “A Benchmark Generator of Tree Decomposition Mk Landscapes”.

## License

Copyright 2021 Tobias van Driessel

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
