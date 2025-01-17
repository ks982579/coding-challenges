cmake_minimum_required(VERSION 3.15)
project(addtwo CXX)




add_library(addtwo src/addtwo.cpp)
target_include_directories(addtwo PUBLIC include)



set_target_properties(addtwo PROPERTIES PUBLIC_HEADER "include/addtwo.h")
install(TARGETS addtwo)
