#include "flood_fill.h"
#include "testframework.h"
#include <iostream>
#include <vector>

int main() {
  std::cout << "Testing ..." << std::endl;
  // using Image = std::vector<std::vector<int>>;
  TestFramework tf;

  tf.addTest("Testing Quacky Test Framework", [&tf]() -> void {
    Image image = {{0}, {0}};
    // std::vector<int> nums({-1, 0, 3, 5, 9, 12});
    std::cout << "In Test" << std::endl;
    Image act = Solution::floodFill(image, 1, 1, 1);
    tf.assertEqual(42, 42);
  });

  tf.run();

  return 0;
}
