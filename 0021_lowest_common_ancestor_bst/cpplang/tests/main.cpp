#include "bin_search_tree.h"
#include "testframework.h"
#include <iostream>
#include <vector>

int main() {
  std::cout << "Testing ..." << std::endl;
  // using Image = std::vector<std::vector<int>>;
  TestFramework tf;

  tf.addTest("Handles null pointers", [&tf]() -> void {
    std::cout << "Adding null pointer test" << std::endl;
    TreeNode *actual =
        Solution::lowestCommonAncestor(nullptr, nullptr, nullptr);
    TreeNode *expected = nullptr;
    tf.assertEqual(expected, actual);
  });

  tf.addTest("Standard first test", [&tf]() -> void {
    TreeNode *tr = new TreeNode(6);
    tr->left = new TreeNode(2);
    tr->left->left = new TreeNode(0);
    tr->left->right = new TreeNode(4);
    tr->left->right->left = new TreeNode(3);
    tr->left->right->right = new TreeNode(5);
    tr->right = new TreeNode(8);
    tr->right->left = new TreeNode(7);
    tr->right->right = new TreeNode(9);

    TreeNode *p = new TreeNode(2);
    TreeNode *q = new TreeNode(8);

    TreeNode *actual = Solution::lowestCommonAncestor(tr, p, q);
    TreeNode *expected = new TreeNode(6);
    tf.assertEqual(expected, actual);
  });

  tf.addTest("Deeper nested test", [&tf]() -> void {
    TreeNode *tr = new TreeNode(6);
    tr->left = new TreeNode(2);
    tr->left->left = new TreeNode(0);
    tr->left->right = new TreeNode(4);
    tr->left->right->left = new TreeNode(3);
    tr->left->right->right = new TreeNode(5);
    tr->right = new TreeNode(8);
    tr->right->left = new TreeNode(7);
    tr->right->right = new TreeNode(9);

    TreeNode *p = new TreeNode(0);
    TreeNode *q = new TreeNode(5);

    TreeNode *actual = Solution::lowestCommonAncestor(tr, p, q);
    TreeNode *expected = new TreeNode(2);
    tf.assertEqual(expected, actual);
  });

  tf.addTest("Deepest Nested Test", [&tf]() -> void {
    TreeNode *tr = new TreeNode(6);
    tr->left = new TreeNode(2);
    tr->left->left = new TreeNode(0);
    tr->left->right = new TreeNode(4);
    tr->left->right->left = new TreeNode(3);
    tr->left->right->right = new TreeNode(5);
    tr->right = new TreeNode(8);
    tr->right->left = new TreeNode(7);
    tr->right->right = new TreeNode(9);

    TreeNode *p = new TreeNode(3);
    TreeNode *q = new TreeNode(9);

    TreeNode *actual = Solution::lowestCommonAncestor(tr, p, q);
    TreeNode *expected = new TreeNode(6);
    tf.assertEqual(expected, actual);
  });

  tf.addTest("Same node is ancestor", [&tf]() -> void {
    TreeNode *tr = new TreeNode(6);
    tr->left = new TreeNode(2);
    tr->left->left = new TreeNode(0);
    tr->left->right = new TreeNode(4);
    tr->left->right->left = new TreeNode(3);
    tr->left->right->right = new TreeNode(5);
    tr->right = new TreeNode(8);
    tr->right->left = new TreeNode(7);
    tr->right->right = new TreeNode(9);

    TreeNode *p = new TreeNode(2);
    TreeNode *q = new TreeNode(5);

    TreeNode *actual = Solution::lowestCommonAncestor(tr, p, q);
    TreeNode *expected = new TreeNode(2);
    tf.assertEqual(expected, actual);
  });

  tf.run();

  return 0;
}
