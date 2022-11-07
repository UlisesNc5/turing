cmake_minimum_required(VERSION 3.8)
project(turing VERSION 1.0.0)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_C_STANDARD 11)

include_directories("include" "/usr/include" "dependecies/glad/include" "dependecies/glm" "dependecies")

file(GLOB_RECURSE COMPILATION_UNITS "src/*.c" "src/*.cpp")
add_executable(turing ${COMPILATION_UNITS})
target_link_libraries(turing ${CMAKE_DL_LIBS})
