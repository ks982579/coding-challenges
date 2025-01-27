#include "bin_search_tree.h"
#include <iostream>
#include <queue>
#include <stack>
#include <utility>

TreeNode *Solution::lowestCommonAncestor(TreeNode *root, TreeNode *p,
                                         TreeNode *q) {
  std::cout << "In Lowest Common Ancestor Method" << std::endl;
  // Handling null pointers.
  if (root == nullptr || p == nullptr || q == nullptr) {
    std::cout << "Leaving Lowest Common Ancestor Method" << std::endl;
    return nullptr;
  }
  // find the one and save each parent in a list.
  // Logic being checking parents on the same level.
  std::stack<TreeNode *> p_parents;
  p_parents.push(root);
  std::stack<TreeNode *> q_parents;
  q_parents.push(root);

  while (true) {
    // pushing a pointer to node.
    std::cout << "P " << p_parents.top()->val << std::endl;
    if (p_parents.top()->val == p->val) {
      break;
    } else if (p_parents.top()->val < p->val) {
      p_parents.push(p_parents.top()->right);
    } else {
      p_parents.push(p_parents.top()->left);
    }
  };

  std::cout << "Leaving Lowest Common Ancestor Method" << std::endl;
  return nullptr;
};
