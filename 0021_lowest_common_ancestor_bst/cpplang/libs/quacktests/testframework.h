#pragma once
#include "bin_search_tree.h"
#include <exception>
#include <functional>
#include <iostream>
#include <ostream>
#include <sstream>
#include <string>
#include <type_traits>
#include <utility>
#include <vector>

using Image = std::vector<std::vector<int>>;

/* To prevent One Definition Rule (ODR) violations we use the `inline` keyword.
 * This tells the compiler to insert the function's code where it is called
 * instead of creating a separate function call. This can prevent multiple
 * definition errors in the same header.
 */
inline std::ostream &operator<<(std::ostream &os, const Image &img) {
  os << "{";

  for (const auto &row : img) {
    os << "{";
    for (const auto &pixel : row) {
      os << pixel << ",";
    }
    os << "}";
  }
  os << "}";
  return os;
};

// This was a failed idea
inline std::string to_string(const std::vector<std::vector<int>> &img) {
  std::stringstream ss;
  ss << img;
  return ss.str();
};

class TestFramework {
  using TestFn = std::function<void()>;
  struct Test {
    std::string name;
    TestFn fn;
  };
  std::vector<Test> tests;
  int passed = 0, failed = 0;

public:
  // Using F&& is a 'universal reference', or a 'forwarding reference'.
  // Combined with `forward<F>`, it preserves the value category of the passed
  // function. that is both lvalues and rvalues!
  // Note: rvalues are like "temporary" values until stored.
  // For mental model, temporary storage space is called "temporary object
  // pool".
  template <typename F> void addTest(const std::string &name, F &&func) {
    tests.push_back({name, std::forward<F>(func)});
  };

  void assertEqual(const Image &expected, const Image &actual) {
    std::cout << "IS " << expected << " == " << actual << " ??" << std::endl;
    if (expected != actual) {
      std::stringstream ss;
      ss << "\nExpected: " << expected << "\nGot: " << actual;
      throw std::runtime_error(ss.str());
    }
  };

  void assertEqual(const TreeNode *expected, const TreeNode *actual) {
    if (expected == nullptr && actual == nullptr) {
      std::cout << "Both are null pointers" << std::endl;
      return;
    } else if (expected == nullptr || actual == nullptr) {
      std::stringstream ss;
      ss << "Something is null";
      // ss << "\nExpected: " << expected->val << "\nGot: " << actual->val;
      throw std::runtime_error(ss.str());
    }
    std::cout << "IS " << expected->val << " == " << actual->val << " ??"
              << std::endl;
    if (expected->val != actual->val) {
      std::stringstream ss;
      ss << "\nExpected: " << expected->val << "\nGot: " << actual->val;
      throw std::runtime_error(ss.str());
    }
  };

  // Templates need definition visible at compile time
  // The template was handling pointers to TreeNode, so removing pointer
  // handling capabilities.
  template <typename T>
  std::enable_if_t<!std::is_pointer_v<T>> assertEqual(const T &expected,
                                                      const T &actual) {
    std::cout << "IS " << expected << " == " << actual << " ??" << std::endl;
    if (expected != actual) {
      throw std::runtime_error("\nExpected: " + std::to_string(expected) +
                               "\nGot: " + std::to_string(actual));
    }
  };

  void run();
};
