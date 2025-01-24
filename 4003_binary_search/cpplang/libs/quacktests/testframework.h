#include <functional>
#include <string>
#include <utility>
#include <vector>

class TestFramework {
  struct Test {
    std::string name;
    std::function<void()> fn;
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
  template <typename F> void addTest(const std::string &name, F &&func);

  template <typename T>
  void assertEqual(const T &expected, const T &actual,
                   const std::string &message = "");

  void run();
};
