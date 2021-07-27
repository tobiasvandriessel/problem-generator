#include <stdio.h>
#include "problem_generator.h"

int main() {
    printf("Hello world from C!\n");
    InputParameters inputParameters;
    inputParameters.m = 5;
    inputParameters.k = 3;
    inputParameters.o = 1;
    inputParameters.b = 2;

    CodomainFunction codomainFunction;
    codomainFunction.tag = DeceptiveTrap; 

    CliqueTree* cliqueTree = construct_clique_tree(inputParameters, codomainFunction);
    free_clique_tree(cliqueTree);
}