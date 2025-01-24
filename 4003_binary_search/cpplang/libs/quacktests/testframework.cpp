#include <exception>
#include <functional>
#include <iostream>
#include <stdexcept>
#include <string>
#include <utility>
#include <vector>

#include "testframework.h"

// void TestFramework::addTest(const std::string &name, TestFn func) {
//   tests.push_back({name, func});
// };

// template <typename T>
// void TestFramework::assertEqual(const T &expected, const T &actual) {
//   std::cout << "IS " << expected << " == " << actual << " ??" << std::endl;
//   if (expected != actual) {
//     throw std::runtime_error("\nExpected: " + std::to_string(expected) +
//                              "\nGot: " + std::to_string(actual));
//   }
// };

void TestFramework::run() {
  for (const auto &test : tests) {
    std::cout << "Running " << test.name << "... ";
    try {
      test.fn();
      std::cout << "PASSED\n";
      passed++;
    } catch (const std::exception &e) {
      std::cout << "FAILED\n" << "   " << e.what() << "\n";
      failed++;
    }
  }
  std::cout << "\nRESULTS: " << passed << " passed, " << failed << " failed\n";
};
