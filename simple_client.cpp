#include <iostream>
#include <algorithm>
#include <vector>
#include "problem_generator.h"

using namespace std;

const double FITNESS_EPSILON = 0.0000000001;


class CliqueTreeC {
    private: 
        CliqueTree* cliqueTree;

        double globOptScore;
        std::vector<std::vector<int>> globOptimaVector;

        std::vector<std::vector<int>> getGlobalOptima(uintptr_t numGlobOpt, uintptr_t length);
        bool isGlobalOptimum(const std::vector<int> &x, double score);

    public:
        bool globalOptimumFound;

        CliqueTreeC(InputParameters inputParameters, CodomainFunction codomainFunction );
        ~CliqueTreeC();

        double evaluate(const std::vector<int> &x);
};

int main() {
    InputParameters inputParameters = InputParameters();
    inputParameters.m = 9;
    inputParameters.k = 3;
    inputParameters.o = 2;
    inputParameters.b = 2;

    CodomainFunction codomainFunction = CodomainFunction();
    codomainFunction.tag = CodomainFunction::Tag::DeceptiveTrap; 

    const std::vector<int> x = {0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0};

    CliqueTreeC cliqueTree(inputParameters, codomainFunction);

    double fitness = cliqueTree.evaluate(x);

    // cout << "Fitness: " << fitness << endl;

}

CliqueTreeC::CliqueTreeC(InputParameters inputParameters, CodomainFunction codomainFunction ) {

    uintptr_t length = (inputParameters.m - 1) * (inputParameters.k - inputParameters.o) + inputParameters.k;

    this->cliqueTree = construct_clique_tree(inputParameters, codomainFunction);

    uintptr_t num_glob_opt = get_number_of_global_optima(this->cliqueTree);
    this->globOptScore = get_score_of_global_optima(this->cliqueTree);

    this->globOptimaVector = this->getGlobalOptima(num_glob_opt, length);
    this->globalOptimumFound = false;
}

CliqueTreeC::~CliqueTreeC(){
    free_clique_tree(this->cliqueTree);
}

double CliqueTreeC::evaluate(const std::vector<int> &x) {
    double fitness = evaluate_solution(this->cliqueTree, x.data(), x.size());
    bool globalOptimumFound = this->isGlobalOptimum(x, fitness);
    return globalOptimumFound;
}

// TODO: Maybe use std::set to find whether the global optima vector/set contains the given solution much faster in the isGlobalOptimum function. 
//   Note that it first must be close to the optimal value, however there might still be a lot of global optima.
std::vector<std::vector<int>> CliqueTreeC::getGlobalOptima(uintptr_t numGlobOpt, uintptr_t length) {

    int** glob_optima_solutions = new int*[numGlobOpt];
    for(int i = 0; i < numGlobOpt; i++) {
        glob_optima_solutions[i] = new int[length];
    }

    write_global_optima_to_pointer(this->cliqueTree, glob_optima_solutions);
    std::vector<std::vector<int>> glob_optima_vector;

    for(int i = 0; i < numGlobOpt; i++) {
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

    for(int i = 0; i < numGlobOpt; i++) {
        delete [] glob_optima_solutions[i];
    }
    delete [] glob_optima_solutions;

    return glob_optima_vector;
}


// TODO: Maybe use std::set to find whether the global optima vector/set contains the given solution much faster. 
//   Note that it first must be close to the optimal value, however there might still be a lot of global optima.
bool CliqueTreeC::isGlobalOptimum(const std::vector<int> &x, double score) {

    return (score == this->globOptScore || (
        std::abs(score - this->globOptScore) < FITNESS_EPSILON && std::find(globOptimaVector.begin(), globOptimaVector.end(), x) != globOptimaVector.end()
    ));
}