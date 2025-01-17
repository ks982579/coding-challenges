#include "addtwo.h"
#include <iostream>

int ListNode::toInt() {
  int storage{0};
  int multiplier{1};

  ListNode *lptr = this; // current node
  while (lptr != nullptr) {
    storage += lptr->val * multiplier;
    multiplier *= 10;
    lptr = lptr->next;
  }

  return storage;
}

ListNode *Solution::addTwoNumbers(ListNode *l1, ListNode *l2) {
  // Copy Pointers - to not break those lists
  ListNode *cl1 = l1;
  ListNode *cl2 = l2;

  // Don't like setting to null...
  ListNode *result = nullptr;
  ListNode *digit = result;

  int carryover{0};

  while ((cl1 != nullptr && cl2 != nullptr) || carryover != 0) {
    int tmp{0};
    tmp += (cl1 != nullptr) ? cl1->val : 0;
    tmp += (cl2 != nullptr) ? cl2->val : 0;

    // Handle Carryover values
    tmp += carryover;
    carryover = 0;

    // Check if there is carry over
    if (tmp > 9) {
      carryover += 1;
      tmp -= 10;
    }

    if (result == nullptr) {
      result = new ListNode{tmp};
    } else {
      digit->next = new ListNode{tmp};
      digit = digit->next;
    }

    // Advance pointers
    cl1 = (cl1 != nullptr) ? cl1->next : nullptr;
    cl2 = (cl2 != nullptr) ? cl2->next : nullptr;
  }
  return result;
}
