cmake_minimum_required(VERSION 3.10)
project(stocktrade)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# CMAKE_BINARY_DIR = where you run from
# CMAKE_SOURCE_DIR = where cmakelists.txt is

# Put output in 'target' directory
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/target)

# Compiled Shared libraries
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/target)

# Static libraries
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/target)

# add libraries
add_library(mylibs
  src/libs/asset_reader.cpp
  src/libs/asset_reader.h
)

# Executable
add_executable(${PROJECT_NAME} src/main.cpp)

# LINKING!
target_link_libraries(${PROJECT_NAME} PRIVATE mylibs)

# Need to copy in the assets as well
add_custom_command(
  TARGET ${PROJECT_NAME} POST_BUILD
  COMMAND ${CMAKE_COMMAND} -E copy_directory
  ${CMAKE_SOURCE_DIR}/assets
  ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/assets
)
