#include <iostream>
#include <vector>
#include "problem_generator.h"

using namespace std;

int main() {
    cout << "Hello world" << endl;
    InputParameters inputParameters = InputParameters();
    inputParameters.m = 9;
    inputParameters.k = 3;
    inputParameters.o = 2;
    inputParameters.b = 2;

    uintptr_t length = (inputParameters.m - 1) * (inputParameters.k - inputParameters.o) + inputParameters.k;

    CodomainFunction codomainFunction = CodomainFunction();
    codomainFunction.tag = CodomainFunction::Tag::DeceptiveTrap; 

    CliqueTree* cliqueTree = construct_clique_tree(inputParameters, codomainFunction);
    const std::vector<int> x = {0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0};

    double fitness = evaluate_solution(cliqueTree, x.data(), x.size());
    // double fitness = evaluate_solution(cliqueTree, solution, length);
    cout << "Fitness: " << fitness << endl;

    uintptr_t num_glob_opt = get_number_of_global_optima(cliqueTree);
    double glob_opt_score = get_score_of_global_optima(cliqueTree);

    int** glob_optima_solutions = new int*[num_glob_opt];
    for(int i = 0; i < num_glob_opt; i++) {
        glob_optima_solutions[i] = new int[length];
    }
    write_global_optima_to_pointer(cliqueTree, glob_optima_solutions);

    for(int i = 0; i < num_glob_opt; i++) {
        cout << "global optima " << i << ": " << endl;
        for(int j = 0; j < length; j++) {
            cout << glob_optima_solutions[i][j];
        }
        cout << endl;
    }
    free_clique_tree(cliqueTree);
}


double evaluate (const std::vector<int> &x) {
    return 1.0;
}