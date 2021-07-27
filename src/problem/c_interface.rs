use std::slice;

use super::{clique_tree::{CliqueTree, InputParameters}, codomain::generate_codomain, codomain_subclasses::CodomainFunction};

#[no_mangle]
pub extern "C" fn construct_clique_tree(
    input_parameters: InputParameters,
    codomain_function: CodomainFunction
) -> *mut CliqueTree {
    let codomain_values = generate_codomain(&input_parameters, &codomain_function);
    let clique_tree = CliqueTree::new(input_parameters, codomain_function, codomain_values);
    Box::into_raw(Box::new(clique_tree))
}

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