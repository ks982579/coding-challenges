// test_package/src/example.cpp
#include "addtwo.h"
#include <iostream>
#include <string>
#include <vector>

int main() {
  // 342 as [2,4,3]
  ListNode *l1 = new ListNode{2};
  l1->next = new ListNode{4};
  l1->next->next = new ListNode{3};

  // 465 as [5,6,4]
  ListNode *l2 = new ListNode{5};
  l2->next = new ListNode{6};
  l2->next->next = new ListNode{4};

  std::cout << "L1 = " << l1->toInt() << "\n";
  std::cout << "L2 = " << l2->toInt() << std::endl;

  ListNode *result = Solution::addTwoNumbers(l1, l2);

  std::cout << "L1 = " << l1->toInt() << "\n";
  std::cout << "L2 = " << l2->toInt() << "\n";
  std::cout << "Res = " << result->toInt() << std::endl;

  delete l1;
  delete l2;
  delete result;
}
