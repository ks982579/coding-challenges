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
