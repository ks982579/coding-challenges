#include "binary_search.h"
#include "testframework.h"
#include <iostream>
#include <vector>

int main() {
  std::cout << "Testing ..." << std::endl;
  TestFramework tf;

  tf.addTest("Find existing Element", [&tf]() -> void {
    std::vector<int> nums({-1, 0, 3, 5, 9, 12});
    int act = Solution::search(nums, 9);
    tf.assertEqual(act, 4);
  });

  tf.addTest("Find existing Element", [&tf]() -> void {
    std::vector<int> nums({-10, 0, 3, 5, 9, 12});
    int act = Solution::search(nums, 3);
    tf.assertEqual(act, 2);
  });

  tf.addTest("Find existing Element", [&tf]() -> void {
    std::vector<int> nums({-1, 0, 3, 5, 9, 12});
    int act = Solution::search(nums, 42);
    tf.assertEqual(act, -1);
  });

  tf.run();

  return 0;
}
