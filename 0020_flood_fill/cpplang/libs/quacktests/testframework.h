#include <exception>
#include <functional>
#include <iostream>
#include <ostream>
#include <sstream>
#include <string>
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

  // Templates need definition visible at compile time
  template <typename T> void assertEqual(const T &expected, const T &actual) {
    std::cout << "IS " << expected << " == " << actual << " ??" << std::endl;
    if (expected != actual) {
      throw std::runtime_error("\nExpected: " + std::to_string(expected) +
                               "\nGot: " + std::to_string(actual));
    }
  };

  void assertEqual(const Image &expected, const Image &actual) {
    std::cout << "IS " << expected << " == " << actual << " ??" << std::endl;
    if (expected != actual) {
      std::stringstream ss;
      ss << "\nExpected: " << expected << "\nGot: " << actual;
      throw std::runtime_error(ss.str());
    }
  };

  void run();
};
