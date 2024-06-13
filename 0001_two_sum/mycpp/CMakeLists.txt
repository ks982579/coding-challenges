cmake_minimum_required(VERSION 3.15)
project(twosum CXX)




add_library(twosum src/twosum.cpp)
target_include_directories(twosum PUBLIC include)



set_target_properties(twosum PROPERTIES PUBLIC_HEADER "include/twosum.h")
install(TARGETS twosum)
