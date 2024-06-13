#pragma once

#include <vector>
#include <string>


#ifdef _WIN32
  #define TWOSUM_EXPORT __declspec(dllexport)
#else
  #define TWOSUM_EXPORT
#endif

TWOSUM_EXPORT void twosum();
TWOSUM_EXPORT void twosum_print_vector(const std::vector<std::string> &strings);
