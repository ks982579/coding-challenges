#include "bin_search_tree.h"
#include <iostream>
#include <stack>

/* Note: Simpler solution would be to work your way down
 *   and just return the point of divergence.
 *   It is a fun property of binary trees.
 */

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
    if (p_parents.top() == nullptr) {
      // This is case where value not in tree.
      // Note: should be error or exception.
      return nullptr;
    }
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

  while (true) {
    if (q_parents.top() == nullptr) {
      // This is case where value not in tree.
      // Note: should be error or exception.
      return nullptr;
    }
    // pushing a pointer to node.
    std::cout << "Q " << q_parents.top()->val << std::endl;

    if (q_parents.top()->val == q->val) {
      break;
    } else if (q_parents.top()->val < q->val) {
      q_parents.push(q_parents.top()->right);
    } else {
      q_parents.push(q_parents.top()->left);
    }
  };

  while (p_parents.size() != q_parents.size()) {
    if (p_parents.size() > q_parents.size()) {
      p_parents.pop();
    } else {
      q_parents.pop();
    }
  };

  while (p_parents.size() > 0) {
    if (p_parents.top()->val == q_parents.top()->val) {
      return p_parents.top();
    } else {
      p_parents.pop();
      q_parents.pop();
    }
  }

  std::cout << "Leaving Lowest Common Ancestor Method" << std::endl;
  return nullptr;
};
