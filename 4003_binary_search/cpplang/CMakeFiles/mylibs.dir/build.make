# CMAKE generated file: DO NOT EDIT!
# Generated by "Unix Makefiles" Generator, CMake Version 3.22

# Delete rule output on recipe failure.
.DELETE_ON_ERROR:

#=============================================================================
# Special targets provided by cmake.

# Disable implicit rules so canonical targets will work.
.SUFFIXES:

# Disable VCS-based implicit rules.
% : %,v

# Disable VCS-based implicit rules.
% : RCS/%

# Disable VCS-based implicit rules.
% : RCS/%,v

# Disable VCS-based implicit rules.
% : SCCS/s.%

# Disable VCS-based implicit rules.
% : s.%

.SUFFIXES: .hpux_make_needs_suffix_list

# Command-line flag to silence nested $(MAKE).
$(VERBOSE)MAKESILENT = -s

#Suppress display of executed commands.
$(VERBOSE).SILENT:

# A target that is always out of date.
cmake_force:
.PHONY : cmake_force

#=============================================================================
# Set environment variables for the build.

# The shell in which to execute make rules.
SHELL = /bin/sh

# The CMake executable.
CMAKE_COMMAND = /usr/bin/cmake

# The command to remove a file.
RM = /usr/bin/cmake -E rm -f

# Escaping for special characters.
EQUALS = =

# The top-level source directory on which CMake was run.
CMAKE_SOURCE_DIR = /home/ksull18/code/coding-challenges/4003_binary_search/cpplang

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = /home/ksull18/code/coding-challenges/4003_binary_search/cpplang

# Include any dependencies generated for this target.
include CMakeFiles/mylibs.dir/depend.make
# Include any dependencies generated by the compiler for this target.
include CMakeFiles/mylibs.dir/compiler_depend.make

# Include the progress variables for this target.
include CMakeFiles/mylibs.dir/progress.make

# Include the compile flags for this target's objects.
include CMakeFiles/mylibs.dir/flags.make

CMakeFiles/mylibs.dir/libs/quacktests/testframework.cpp.o: CMakeFiles/mylibs.dir/flags.make
CMakeFiles/mylibs.dir/libs/quacktests/testframework.cpp.o: libs/quacktests/testframework.cpp
CMakeFiles/mylibs.dir/libs/quacktests/testframework.cpp.o: CMakeFiles/mylibs.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/home/ksull18/code/coding-challenges/4003_binary_search/cpplang/CMakeFiles --progress-num=$(CMAKE_PROGRESS_1) "Building CXX object CMakeFiles/mylibs.dir/libs/quacktests/testframework.cpp.o"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/mylibs.dir/libs/quacktests/testframework.cpp.o -MF CMakeFiles/mylibs.dir/libs/quacktests/testframework.cpp.o.d -o CMakeFiles/mylibs.dir/libs/quacktests/testframework.cpp.o -c /home/ksull18/code/coding-challenges/4003_binary_search/cpplang/libs/quacktests/testframework.cpp

CMakeFiles/mylibs.dir/libs/quacktests/testframework.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing CXX source to CMakeFiles/mylibs.dir/libs/quacktests/testframework.cpp.i"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /home/ksull18/code/coding-challenges/4003_binary_search/cpplang/libs/quacktests/testframework.cpp > CMakeFiles/mylibs.dir/libs/quacktests/testframework.cpp.i

CMakeFiles/mylibs.dir/libs/quacktests/testframework.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling CXX source to assembly CMakeFiles/mylibs.dir/libs/quacktests/testframework.cpp.s"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /home/ksull18/code/coding-challenges/4003_binary_search/cpplang/libs/quacktests/testframework.cpp -o CMakeFiles/mylibs.dir/libs/quacktests/testframework.cpp.s

# Object files for target mylibs
mylibs_OBJECTS = \
"CMakeFiles/mylibs.dir/libs/quacktests/testframework.cpp.o"

# External object files for target mylibs
mylibs_EXTERNAL_OBJECTS =

target/libmylibs.a: CMakeFiles/mylibs.dir/libs/quacktests/testframework.cpp.o
target/libmylibs.a: CMakeFiles/mylibs.dir/build.make
target/libmylibs.a: CMakeFiles/mylibs.dir/link.txt
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --bold --progress-dir=/home/ksull18/code/coding-challenges/4003_binary_search/cpplang/CMakeFiles --progress-num=$(CMAKE_PROGRESS_2) "Linking CXX static library target/libmylibs.a"
	$(CMAKE_COMMAND) -P CMakeFiles/mylibs.dir/cmake_clean_target.cmake
	$(CMAKE_COMMAND) -E cmake_link_script CMakeFiles/mylibs.dir/link.txt --verbose=$(VERBOSE)

# Rule to build all files generated by this target.
CMakeFiles/mylibs.dir/build: target/libmylibs.a
.PHONY : CMakeFiles/mylibs.dir/build

CMakeFiles/mylibs.dir/clean:
	$(CMAKE_COMMAND) -P CMakeFiles/mylibs.dir/cmake_clean.cmake
.PHONY : CMakeFiles/mylibs.dir/clean

CMakeFiles/mylibs.dir/depend:
	cd /home/ksull18/code/coding-challenges/4003_binary_search/cpplang && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /home/ksull18/code/coding-challenges/4003_binary_search/cpplang /home/ksull18/code/coding-challenges/4003_binary_search/cpplang /home/ksull18/code/coding-challenges/4003_binary_search/cpplang /home/ksull18/code/coding-challenges/4003_binary_search/cpplang /home/ksull18/code/coding-challenges/4003_binary_search/cpplang/CMakeFiles/mylibs.dir/DependInfo.cmake --color=$(COLOR)
.PHONY : CMakeFiles/mylibs.dir/depend

