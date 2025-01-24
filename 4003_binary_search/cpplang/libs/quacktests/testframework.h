#include <exception>
#include <functional>
#include <iostream>
#include <string>
#include <utility>
#include <vector>

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

  void run();
};
