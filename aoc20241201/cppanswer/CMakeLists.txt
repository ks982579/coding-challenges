cmake_minimum_required(VERSION 3.10)
project(aoc2024)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# CMAKE_BINARY_DIR = where you run from
# CMAKE_SOURCE_DIR = where CMakeLists.txt is
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/target)
# for compiled shared libraries
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/target)
# for static libraries
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/target)

add_library(mylibs
  src/libs/asset_reader.cpp
  src/libs/asset_reader.h
)

add_executable(${PROJECT_NAME} src/main.cpp)

# Linking Libraries
target_link_libraries(${PROJECT_NAME} PRIVATE mylibs)

add_custom_command(
  TARGET ${PROJECT_NAME} POST_BUILD
  COMMAND ${CMAKE_COMMAND} -E copy_directory
  ${CMAKE_SOURCE_DIR}/assets
  ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/assets
)
