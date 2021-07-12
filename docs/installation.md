## Documentation Pages

- [Index](index.md)
  - [Subcommands](subcommands.md)
  - [File Structures](file_structures.md)
  - [Installation](installation.md)

# Installation

## Binary

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