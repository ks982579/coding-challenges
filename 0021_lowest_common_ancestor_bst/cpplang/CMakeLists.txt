cmake_minimum_required(VERSION 3.10)
project(lcabsttest)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# CMAKE_BINARY_DIR = where you run from
# CMAKE_SOURCE_DIR = where cmakelists.txt is

# Put output in 'target' directory
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/target)

# Compiled shared libraries
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/target)

# Static libraries
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/target)

# add libraries
add_library(
  quacktests
  libs/quacktests/testframework.cpp
)

add_library(
  binsearch
  libs/binsearch/bin_search_tree.cpp
)

# add_library(
#   mylibs
#   # src/libs/asset_reader.cpp
#   # src/libs/asset_reader.h
#   # libs/binsearch/binary_search.cpp
# )

# Executable
add_executable(${PROJECT_NAME} tests/main.cpp)

# PUBLIC = Library and users need headers
# PRIVATE = only libraries need headers
# LINKING
target_include_directories(quacktests PUBLIC libs/quacktests)
target_include_directories(binsearch PUBLIC libs/binsearch)
target_link_libraries(quacktests PUBLIC binsearch)
target_link_libraries(${PROJECT_NAME} PRIVATE quacktests binsearch)


# Need to copy in the assets as well
# add_custom_command(
#   TARGET ${PROJECT_NAME} POST_BUILD
#   COMMAND ${CMAKE_COMMAND} -E copy_directory
#   ${CMAKE_SOURCE_DIR}/assets
#   ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/assets
# )
