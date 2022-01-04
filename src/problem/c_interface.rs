use std::slice;

use rand_chacha::ChaChaRng;

use super::{clique_tree::{CliqueTree, InputParameters}, codomain::generate_codomain, codomain_subclasses::CodomainFunction, configuration::get_rng};


/// Get a random number generator, required for generating codomain values or constructing clique trees.
/// If the seed parameter is a null pointer, a random seed will be generated.
#[no_mangle]
pub extern "C" fn get_rng_c(
    seed: Option<&u64>,
) -> *mut ChaChaRng {
    let seed_new = seed.map(|&x| x);
    let rng = get_rng(seed_new);
    Box::into_raw(Box::new(rng))
}

/// Construct CliqueTree (which represents the TD Mk Landscape) using the input parameters (M, k, o, b) 
///   and the codomain function to be used to generate the codomain. 
/// It returns a pointer to the (opaque) CliqueTree struct, which we can subsequently use to evaluate solutions, 
///   get the global optima, and drop/destruct the CliqueTree. 
#[no_mangle]
pub extern "C" fn construct_clique_tree(
    input_parameters: InputParameters,
    codomain_function: CodomainFunction, 
    rng_ptr: *mut ChaChaRng,
) -> *mut CliqueTree { 
    let rng = unsafe {
        assert!(!rng_ptr.is_null());
        &mut *rng_ptr
    };
    let codomain_values = generate_codomain(&input_parameters, &codomain_function, rng);
    let clique_tree = CliqueTree::new(input_parameters, codomain_function, codomain_values, rng);
    Box::into_raw(Box::new(clique_tree))
}

/// Get a Rust vector with the codomain from a 2D pointer array.
/// Importantly, the codomain that was passed (using the pointer) can be freed/deleted, as we copy the codomain.
fn get_vector_codomain_from_pointer(
    input_parameters: &InputParameters,
    codomain: *const *const f64,
) -> Vec<Vec<f64>> {

    let mut result_codomain = vec![];


    //The codomain has M (number of cliques) entries
    let all_codomain = unsafe {
        assert!(!codomain.is_null());
        slice::from_raw_parts(codomain, input_parameters.m as usize)
    };

    //And the codomain for each clique has 2^k entries, or equivelantly, 1 << k
    for i in 0..input_parameters.m as usize {

        let clique_codomain = unsafe {
            assert!(!all_codomain[i].is_null());
            // We use 1 << k here, as the number of entries in the 
            slice::from_raw_parts(all_codomain[i], (1 << input_parameters.k) as usize)
        };

        //Construct vector from the slice/array and push it to the result vector
        let clique_codomain_vec = Vec::from(clique_codomain);
        result_codomain.push(clique_codomain_vec);
    }
    result_codomain
}

/// Construct CliqueTree (which represents the TD Mk Landscape) using the input parameters (M, k, o, b) 
///   and the custom codomain values. The codomain has 2^k entries for each clique (out of M in total).
/// It returns a pointer to the (opaque) CliqueTree struct, which we can subsequently use to evaluate solutions, 
///   get the global optima, and drop/destruct the CliqueTree. 
/// Importantly, the codomain that was passed (using the pointer) can be freed/deleted, as we copy the codomain.
#[no_mangle]
pub extern "C" fn construct_clique_tree_custom_codomain(
    input_parameters: InputParameters,
    codomain: *const *const f64,
    rng_ptr: *mut ChaChaRng,
) -> *mut CliqueTree { 
    let rng = unsafe {
        assert!(!rng_ptr.is_null());
        &mut *rng_ptr
    };
    //First copy the 2D pointer array for the codomain into a vector of vectors (which is necessary for our CliqueTree constructor)
    let codomain_values = get_vector_codomain_from_pointer(&input_parameters, codomain);

    let clique_tree = CliqueTree::new(input_parameters, CodomainFunction::Unknown, codomain_values, rng);
    Box::into_raw(Box::new(clique_tree))
}

/// Delete/drop/free the CliqueTree struct (memory)
#[no_mangle]
pub extern "C" fn free_clique_tree(
    clique_tree_ptr: *mut CliqueTree,
) {
    if clique_tree_ptr.is_null() {
        return;
    } 
    unsafe {
        Box::from_raw(clique_tree_ptr);
    }
}


/// Evaluate a given solution
#[no_mangle]
pub extern "C" fn evaluate_solution(
    clique_tree_ptr: *mut CliqueTree,
    solution: *const i32,
    len: usize
) -> f64 {
    let clique_tree = unsafe {
        assert!(!clique_tree_ptr.is_null());
        &*clique_tree_ptr
    };
    let solution_slice = unsafe {
        assert!(!solution.is_null());

        slice::from_raw_parts(solution, len)
    };

    let mut num_eval = 0;
    clique_tree.calculate_fitness_int(solution_slice, &mut num_eval)
}

/// Get the number of global optima for this TD Mk Landscape problem
#[no_mangle]
pub extern "C" fn get_number_of_global_optima(
    clique_tree_ptr: *mut CliqueTree,
) -> usize {
    let clique_tree = unsafe {
        assert!(!clique_tree_ptr.is_null());
        &*clique_tree_ptr
    };
    clique_tree.glob_optima_strings.len()
}

/// Get the global optimum/optima score
#[no_mangle]
pub extern "C" fn get_score_of_global_optima(
    clique_tree_ptr: *mut CliqueTree,
) -> f64 {
    let clique_tree = unsafe {
        assert!(!clique_tree_ptr.is_null());
        &*clique_tree_ptr
    };
    clique_tree.glob_optima_score
}


/// Write the global optima of the problem to the memory pointed to by the passed pointer. 
/// Then, one can check at C/C++ side whether a global optimum has been found
#[no_mangle]
pub extern "C" fn write_global_optima_to_pointer(
    clique_tree_ptr: *mut CliqueTree,
    glob_opt_ptr: *const *mut i32,
) {
    let clique_tree = unsafe {
        assert!(!clique_tree_ptr.is_null());
        &*clique_tree_ptr
    };

    let n = clique_tree.glob_optima_strings[0].len();

    // Convert the pointer into a slice
    let arrays_slice = unsafe {
        assert!(!glob_opt_ptr.is_null());

        slice::from_raw_parts(glob_opt_ptr, clique_tree.glob_optima_strings.len())
    };


    // For each global optimum, 
    for (i, glob_opt) in clique_tree.glob_optima_strings.iter().enumerate() {

        //println!("Rust glob opt {}: \n {:?}", i, glob_opt);
        unsafe {
            //We convert the C++ side pointer into a slice so we can edit it nicely
            let array_slice = {
                assert!(!arrays_slice[i].is_null());

                slice::from_raw_parts_mut(arrays_slice[i], n)
            };

            // and write the global optimum by assigning all element values
            for j in 0..n {
                array_slice[j] = glob_opt[j] as i32;
            }
        }
    }
    
}