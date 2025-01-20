#include "mergelists.h"
#include <string>
#include <vector>

int main() {
  // mergelists();

  // std::vector<std::string> vec;
  // vec.push_back("test_package");
  ListNode *list1 = new ListNode{3};
  list1->next = new ListNode{3};
  list1->next->next = new ListNode{6};
  list1->next->next->next = new ListNode{8};

  ListNode *list2 = new ListNode{4};
  list2->next = new ListNode{5};
  list2->next = new ListNode{7};

  ListNode *result_h = Solution::mergeTwoLists(list1, list2);

  // Copy to destroy
  ListNode *res = result_h;

  while (res != nullptr) {
    std::cout << res->val << "endl";
    res = res->next;
  }

  delete result_h;
};
