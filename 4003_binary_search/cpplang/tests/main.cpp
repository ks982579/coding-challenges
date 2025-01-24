#include "binary_search.h"
#include "testframework.h"
#include <iostream>
#include <vector>

int main() {
  std::cout << "Testing ..." << std::endl;
  TestFramework tf;

  std::vector<int> nums({-1, 0, 3, 5, 9, 12});
  int result = Solution::search(nums, 9);

  tf.addTest("Find existing Element", []() -> void {
    std::vector<int> nums({-1, 0, 3, 5, 9, 12});
    std::cout << "in test" << std::endl;
  });
  // tf.run();

  return 0;
}
