cmake_minimum_required(VERSION 3.15)
project(mergelists CXX)




add_library(mergelists src/mergelists.cpp)
target_include_directories(mergelists PUBLIC include)



set_target_properties(mergelists PROPERTIES PUBLIC_HEADER "include/mergelists.h")
install(TARGETS mergelists)
