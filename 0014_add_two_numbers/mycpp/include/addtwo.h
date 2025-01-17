// include/addtwo.h
#pragma once

#include <string>
#include <vector>

struct ListNode {
  // members of struct
  int val;        // value of this node
  ListNode *next; // pointer to next node

  // Default constructor
  ListNode() : val(0), next(nullptr) {}
  // Constructor with value
  ListNode(int x) : val(x), next(nullptr) {}
  // Constructor that takes value and next pointer.
  ListNode(int x, ListNode *next) : val(x), next(next) {}
  int toInt();

  // Adding destructor
  ~ListNode() { delete next; }
};

class Solution {
public:
  static ListNode *addTwoNumbers(ListNode *l1, ListNode *l2);
};
