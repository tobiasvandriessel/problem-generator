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

    let arrays_slice = unsafe {
        assert!(!glob_opt_ptr.is_null());

        slice::from_raw_parts(glob_opt_ptr, clique_tree.glob_optima_strings.len())
    };


    for (i, glob_opt) in clique_tree.glob_optima_strings.iter().enumerate() {

        //println!("Rust glob opt {}: \n {:?}", i, glob_opt);
        unsafe {
            let array_slice = {
                assert!(!arrays_slice[i].is_null());

                slice::from_raw_parts_mut(arrays_slice[i], n)
            };

            for j in 0..n {
                array_slice[j] = glob_opt[j] as i32;
            }
        }
    }
    
}

//Deze moet dan in een array die is geallocate door c++ de solutions schrijven
// #[no_mangle]
// pub extern "C" fn get_global_optima(
//     clique_tree_ptr: *mut CliqueTree,
// ) -> bool {
//     let clique_tree = unsafe {
//         assert!(!clique_tree_ptr.is_null());
//         &*clique_tree_ptr
//     };



// }