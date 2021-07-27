#include <iostream>
#include "td_mk_landscape.h"

using namespace std;

int main() {
    cout << "Hello world" << endl;
    InputParameters inputParameters = InputParameters();
    inputParameters.m = 5;
    inputParameters.k = 3;
    inputParameters.o = 1;
    inputParameters.b = 2;

    CodomainFunction codomainFunction = CodomainFunction();
    codomainFunction.tag = CodomainFunction::Tag::DeceptiveTrap; 

    CliqueTree* cliqueTree = construct_clique_tree(inputParameters, codomainFunction);
    free_clique_tree(cliqueTree);
}