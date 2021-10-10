# problem-generator/CMakeLists.txt

if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CARGO_CMD cargo build)
    set(TARGET_DIR "debug")
else ()
    set(CARGO_CMD cargo build --release)
    set(TARGET_DIR "release")
endif ()

set(PROBLEM_GENERATOR_SO "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/libproblem_generator.so")

add_custom_target(problem_generator ALL
    COMMENT "Compiling problem generator module"
    COMMAND CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} ${CARGO_CMD} 
    COMMAND cp ${PROBLEM_GENERATOR_SO} ${CMAKE_CURRENT_BINARY_DIR}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
set_target_properties(problem_generator PROPERTIES LOCATION ${CMAKE_CURRENT_BINARY_DIR})

#add_test(NAME problem_generator_test 
#    COMMAND cargo test
#    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
