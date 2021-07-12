## Documentation Pages

- [Index](index.md)
  - [Subcommands](subcommands.md)
  - [File Structures](file_structures.md)
  - [Installation](installation.md)

# File Structures

- [File Structures](#file-structures)
  - [Configuration file](#configuration-file)
  - [Codomain File Structure](#codomain-file-structure)
  - [Problem File Structure](#problem-file-structure)

## Configuration file


The input configuration file is used to generate deceptive trap problems with topology parameters in a range. It has the following structure:
```
    M INCL_START_M EXCL_END_M
    k INCL_START_K EXCL_END_K
    o INCL_START_O EXCL_END_O
    b INCL_START_B EXCL_END_B
    CODOMAIN_CLASS [CODOMAIN_CLASS_PAR...]
```
where `M`, `k`, `o`, and `b` are literals and `INCL_START_X` and `EXCL_END_X` represent the to be inserted values of the start (incl.) and end (excl.) values for that variable `X`. `CODOMAIN_CLASS` is the used codomain class and `CODOMAIN_CLASS_PAR` are any parameters for the codomain class.

For example, if we use $M \in \{1, ..., 49\}$, $k = 5$, $o = 1$, $b = 1$, and the deceptive trap codomain function: 
```
    M 1 50 
    k 5 6 
    o 1 2 
    b 1 2
    deceptive-trap
```

As options for the codomain we currently offer: *Random*, *Deceptive Trap*, *NKq*, *NKp*, and *Random Deceptive Trap* (a combination of the two). Here we have chosen the deceptive trap function. Note that the deceptive trap codomain function has a randomly generated local optimum and deceptive attractor (its inverse).

## Codomain File Structure

The input codomain files should have the following structure: 
```
    M K O B
    CODOMAIN_VALUE_1
    ...
    CODOMAIN_VALUE_LAST
```
where `M`, `K`, `O`, and `B` represent the to be inserted values of $M$, $k$, $o$ and $b$, and `CODOMAIN_VALUE_1` `...` `CODOMAIN_VALUE_LAST` represent the $M \cdot 2^k$ decimal codomain values, each on a new line. 

## Problem File Structure

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