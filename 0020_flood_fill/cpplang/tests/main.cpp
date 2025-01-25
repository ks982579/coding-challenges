#include "flood_fill.h"
#include "testframework.h"
#include <iostream>
#include <vector>

int main() {
  std::cout << "Testing ..." << std::endl;
  // using Image = std::vector<std::vector<int>>;
  TestFramework tf;

  tf.addTest("Testing Quacky Test Framework", [&tf]() -> void {
    Image image = {{1, 1, 1}, {1, 1, 0}, {1, 0, 1}};
    // std::vector<int> nums({-1, 0, 3, 5, 9, 12});
    std::cout << "In Test" << std::endl;
    Image actual = Solution::floodFill(image, 1, 1, 2);
    Image expected = {{1, 1, 1}, {1, 2, 0}, {1, 0, 1}};
    tf.assertEqual(expected, actual);
  });

  tf.run();

  return 0;
}
