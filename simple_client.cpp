#include <iostream>
#include <algorithm>
#include <set>
#include <vector>
#include "target/problem_generator.h"
#include <stdexcept>

using namespace std;

const double FITNESS_EPSILON = 0.0000000001;

ChaChaRng* chaChaRng;

class CliqueTreeC {
    private: 
        CliqueTree* cliqueTree;

        double globOptScore;
        std::set<std::vector<int>> globOptimaSet;

        std::set<std::vector<int>> getGlobalOptima(uintptr_t numGlobOpt, uintptr_t length);
        bool isGlobalOptimum(const std::vector<int> &x, double score);

    public:
        bool globalOptimumFound;

        CliqueTreeC(InputParameters inputParameters, CodomainFunction codomainFunction, ChaChaRng* chaChaRng);
        ~CliqueTreeC();

        double evaluate(const std::vector<int> &x);
};

int main() {
    InputParameters inputParameters = InputParameters();
    inputParameters.m = 5;
    inputParameters.k = 3;
    inputParameters.o = 1;
    inputParameters.b = 2;

    CodomainFunction codomainFunction = CodomainFunction();
    codomainFunction.tag = CodomainFunction::Tag::DeceptiveTrap; 

    const uint64_t seed = 2398;
    chaChaRng = get_rng_c(&seed);

    CliqueTreeC cliqueTree(inputParameters, codomainFunction, chaChaRng);


    // const std::vector<int> x = {0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0};
    const std::vector<int> x = {0, 0, 0, 1, 0, 1, 1, 1, 1, 0, 0};
    double fitness = cliqueTree.evaluate(x);

    cout << "Fitness: " << fitness << endl;
    cout << "Global optimum found: " << boolalpha << cliqueTree.globalOptimumFound << endl;

}

CliqueTreeC::CliqueTreeC(InputParameters inputParameters, CodomainFunction codomainFunction, ChaChaRng* chaChaRng ) {

    uintptr_t length = (inputParameters.m - 1) * (inputParameters.k - inputParameters.o) + inputParameters.k;

    this->cliqueTree = construct_clique_tree(inputParameters, codomainFunction, chaChaRng);

    uintptr_t num_glob_opt = get_number_of_global_optima(this->cliqueTree);
    this->globOptScore = get_score_of_global_optima(this->cliqueTree);

    this->globOptimaSet = this->getGlobalOptima(num_glob_opt, length);
    this->globalOptimumFound = false;
}

CliqueTreeC::~CliqueTreeC(){
    free_clique_tree(this->cliqueTree);
}

double CliqueTreeC::evaluate(const std::vector<int> &x) {
    double fitness = evaluate_solution(this->cliqueTree, x.data(), x.size());
    this->globalOptimumFound = this->isGlobalOptimum(x, fitness);
    return fitness;
}

std::set<std::vector<int>> CliqueTreeC::getGlobalOptima(uintptr_t numGlobOpt, uintptr_t length) {

    int** globOptimaSolutions = new int*[numGlobOpt];
    for(int i = 0; i < numGlobOpt; i++) {
        globOptimaSolutions[i] = new int[length];
    }

    write_global_optima_to_pointer(this->cliqueTree, globOptimaSolutions);
    std::set<std::vector<int>> globOptimaSet;

    for(int i = 0; i < numGlobOpt; i++) {
        // cout << "global optima " << i << ": " << endl;
        // for(int j = 0; j < length; j++) {
        //     cout << glob_optima_solutions[i][j];
        // }
        // cout << endl;

        std::vector<int> glob_opt(globOptimaSolutions[i], globOptimaSolutions[i] + length);
        // for(int j = 0; j < length; j++) {
        //     cout << glob_opt[j];
        // }
        // cout << endl;
        auto result = globOptimaSet.insert(glob_opt);
        if (!result.second) {
            throw std::logic_error("Global optima are not unique...");
        }

    }

    for(int i = 0; i < numGlobOpt; i++) {
        delete [] globOptimaSolutions[i];
    }
    delete [] globOptimaSolutions;

    return globOptimaSet;
}


bool CliqueTreeC::isGlobalOptimum(const std::vector<int> &x, double score) {

    return (score == this->globOptScore || (
        std::abs(score - this->globOptScore) < FITNESS_EPSILON && this->globOptimaSet.find(x) != this->globOptimaSet.end() 
    ));
}