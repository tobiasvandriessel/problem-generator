# problem-generator/CMakeLists.txt

# Older versions may still work but were not tested.
cmake_minimum_required(VERSION 3.18)

project(problem_generator)

if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CARGO_CMD cargo build)
    set(TARGET_DIR "debug")
else ()
    set(CARGO_CMD cargo build --release)
    set(TARGET_DIR "release")
endif ()
if(WIN32 OR MSYS OR CYGWIN OR MINGW)
set(PROBLEM_GENERATOR_SO "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/libproblem_generator.dll")
endif()



if(UNIX)
    if(APPLE)
        set(PROBLEM_GENERATOR_SO "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/libproblem_generator.dylib")
    else()
        set(PROBLEM_GENERATOR_SO "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/libproblem_generator.so")
    endif()
endif()


add_custom_target(problem_generator ALL
    COMMENT "Compiling problem generator module"
    COMMAND CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} ${CARGO_CMD}
    COMMAND cp ${PROBLEM_GENERATOR_SO} ${CMAKE_CURRENT_BINARY_DIR}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
set_target_properties(problem_generator PROPERTIES LOCATION ${CMAKE_CURRENT_BINARY_DIR})

add_library(problem_generator_lib SHARED IMPORTED GLOBAL)
set_property(TARGET problem_generator_lib PROPERTY IMPORTED_LOCATION ${PROBLEM_GENERATOR_SO})
#add_test(NAME problem_generator_test
#    COMMAND cargo test
#    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
