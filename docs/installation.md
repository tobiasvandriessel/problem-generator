## Documentation Pages

- [Index](index.md)
  - [Subcommands](subcommands.md)
  - [File Structures](file_structures.md)
  - [Installation](installation.md)

# Installation

## Binary

There are multiple options to choose from:

#### A. Download executable:

1. Download executable from the [Release](https://github.com/tobiasvandriessel/problem-generator/releases/latest) page

#### B. Install using crates.io:  

> [!NOTE]
> If you do not have Rust installed yet, run ```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh``` to install [Rustup](https://rustup.rs/) and the Rust programming language along with the package manager, Cargo. 
> 
> Alternatively, use the repo's Nix Flake to set up the dev env automatically, by entering the dev shell manually or by using Nix-direnv.

1. Run ```cargo install problem_generator``` to install the problem generator from [crates.io](https://crates.io/)  

#### C. Compile  
> [!NOTE]
> If you do not have Rust installed yet, run ```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh``` to install [Rustup](https://rustup.rs/) and the Rust programming language along with the package manager, Cargo. 
> 
> Alternatively, use the repo's Nix Flake to set up the dev env automatically, by entering the dev shell manually or by using Nix-direnv.
1. Checkout this repository ```git clone https://github.com/tobiasvandriessel/problem-generator && cd problem-generator```  
1. and run ```cargo install --path .``` or ```cargo build --release```, 
	     depending on whether you want it installed or just built.  