#include "flood_fill.h"
#include "testframework.h"
#include <iostream>
#include <vector>

int main() {
  std::cout << "Testing ..." << std::endl;
  // using Image = std::vector<std::vector<int>>;
  TestFramework tf;

  tf.addTest("Standard first test", [&tf]() -> void {
    Image image = {{1, 1, 1}, {1, 1, 0}, {1, 0, 1}};
    std::cout << "In Test" << std::endl;
    Image actual = Solution::floodFill(image, 1, 1, 2);
    Image expected = {{2, 2, 2}, {2, 2, 0}, {2, 0, 1}};
    tf.assertEqual(expected, actual);
  });

  tf.addTest("No Fill", [&tf]() -> void {
    Image image = {{1, 2, 3, 1}, {4, 3, 3, 2}, {1, 1, 0, 5}, {1, 1, 1, 9}};
    std::cout << "In Test" << std::endl;
    Image actual = Solution::floodFill(image, 2, 2, 8);
    Image expected = {{1, 2, 3, 1}, {4, 3, 3, 2}, {1, 1, 8, 5}, {1, 1, 1, 9}};
    tf.assertEqual(expected, actual);
  });

  tf.run();

  return 0;
}
