#pragma once

#include <vector>
#include <string>


#ifdef _WIN32
  #define ADDTWO_EXPORT __declspec(dllexport)
#else
  #define ADDTWO_EXPORT
#endif

ADDTWO_EXPORT void addtwo();
ADDTWO_EXPORT void addtwo_print_vector(const std::vector<std::string> &strings);
