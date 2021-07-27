#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

///The CliqueTree struct with properties input parameters, clique variable indices, the used codomain function, codomain values, global optimum strings and score
struct CliqueTree;

///Struct to contain the input parameters of the TD Mk Landscape:
/// Number of cliques/subfunctions M,
/// size k of each clique/subfunction,
/// number of overlapping variables between cliques/subfunctions o,
/// number of branches in the clique tree / tree decomposition b
struct InputParameters {
  uint32_t m;
  uint32_t k;
  uint32_t o;
  uint32_t b;
};

///Enum to represent various codomain classes
struct CodomainFunction {
  enum class Tag {
    Random,
    Trap,
    DeceptiveTrap,
    NKq,
    NKp,
    ///Combination of random and deceptive trap, where every clique/subfunction has probability p_deceptive to be deceptive and (1 - p_deceptive) to be random
    RandomDeceptiveTrap,
    Unknown,
  };

  struct NKq_Body {
    uint32_t q;
  };

  struct NKp_Body {
    double p;
  };

  struct RandomDeceptiveTrap_Body {
    double p_deceptive;
  };

  Tag tag;
  union {
    NKq_Body n_kq;
    NKp_Body n_kp;
    RandomDeceptiveTrap_Body random_deceptive_trap;
  };
};

extern "C" {

CliqueTree *construct_clique_tree(InputParameters input_parameters,
                                  CodomainFunction codomain_function);

void free_clique_tree(CliqueTree *clique_tree_ptr);

} // extern "C"
