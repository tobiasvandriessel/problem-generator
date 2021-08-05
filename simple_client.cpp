#include <iostream>
#include <algorithm>
#include <vector>
#include "problem_generator.h"

using namespace std;

const double FITNESS_EPSILON = 0.0000000001;

int main() {
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


    uintptr_t num_glob_opt = get_number_of_global_optima(cliqueTree);
    double glob_opt_score = get_score_of_global_optima(cliqueTree);

    int** glob_optima_solutions = new int*[num_glob_opt];
    for(int i = 0; i < num_glob_opt; i++) {
        glob_optima_solutions[i] = new int[length];
    }

    write_global_optima_to_pointer(cliqueTree, glob_optima_solutions);
    std::vector<std::vector<int>> glob_optima_vector;

    for(int i = 0; i < num_glob_opt; i++) {
        // cout << "global optima " << i << ": " << endl;
        // for(int j = 0; j < length; j++) {
        //     cout << glob_optima_solutions[i][j];
        // }
        // cout << endl;

        std::vector<int> glob_opt(glob_optima_solutions[i], glob_optima_solutions[i] + length);
        // for(int j = 0; j < length; j++) {
        //     cout << glob_opt[j];
        // }
        // cout << endl;
        glob_optima_vector.push_back(glob_opt);
    }

    double fitness = evaluate_solution(cliqueTree, x.data(), x.size());
    bool globalOptimumFound = isGlobalOptimum(glob_opt_score, glob_optima_vector, x, fitness);
    // cout << "Fitness: " << fitness << endl;

    for(int i = 0; i < num_glob_opt; i++) {
        delete [] glob_optima_solutions[i];
    }
    delete [] glob_optima_solutions;

    free_clique_tree(cliqueTree);
}


double evaluate (const std::vector<int> &x) {
    return 1.0;
}

// TODO: Maybe use std::set to find whether the global optima vector/set contains the given solution much faster. 
//   Note that it first must be close to the optimal value, however there might still be a lot of global optima.
bool isGlobalOptimum(double globOptimaScore, const std::vector<std::vector<int>> &globOptimaVector, const std::vector<int> &x, double score) {

    return (score == globOptimaScore || (
        std::abs(score - globOptimaScore) < FITNESS_EPSILON && std::find(globOptimaVector.begin(), globOptimaVector.end(), x) != globOptimaVector.end()
    ));
}