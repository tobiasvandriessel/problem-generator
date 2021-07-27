#include <iostream>
#include "problem_generator.h"

using namespace std;

int main() {
    cout << "Hello world" << endl;
    InputParameters inputParameters = InputParameters();
    inputParameters.m = 5;
    inputParameters.k = 3;
    inputParameters.o = 1;
    inputParameters.b = 2;

    uintptr_t length = (inputParameters.m - 1) * (inputParameters.k - inputParameters.o) + inputParameters.k;
    uint32_t solution[11] = {0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0};

    CodomainFunction codomainFunction = CodomainFunction();
    codomainFunction.tag = CodomainFunction::Tag::DeceptiveTrap; 

    CliqueTree* cliqueTree = construct_clique_tree(inputParameters, codomainFunction);

    double fitness = evaluate_solution(cliqueTree, solution, length);
    cout << "Fitness: " << fitness << endl;

    free_clique_tree(cliqueTree);
}