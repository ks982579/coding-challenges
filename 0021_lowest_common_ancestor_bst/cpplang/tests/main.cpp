#include "bin_search_tree.h"
#include "testframework.h"
#include <iostream>
#include <vector>

int main() {
  std::cout << "Testing ..." << std::endl;
  // using Image = std::vector<std::vector<int>>;
  TestFramework tf;

  tf.addTest("Standard first test", [&tf]() -> void {
    TreeNode *tr(nullptr);
    std::cout << "In Test" << std::endl;
    TreeNode *actual =
        Solution::lowestCommonAncestor(nullptr, nullptr, nullptr);
    TreeNode *expected = nullptr;
    tf.assertEqual(expected, actual);
  });

  tf.run();

  return 0;
}
