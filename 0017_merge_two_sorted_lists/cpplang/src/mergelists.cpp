#include "mergelists.h"
#include <iostream>

ListNode *Solution::mergeTwoLists(ListNode *list1, ListNode *list2) {
  std::cout << "Hello There" << std::endl;

  ListNode *result_head = nullptr;
  ListNode *current_node = nullptr;

  while (list1 != nullptr || list2 != nullptr) {
    std::cout << "top of loop" << std::endl;
    // There should be a better way to organize statements
    // abstract the details inside each check, logic is same.
    if (list1 == nullptr) {
      // wouldn't be in loop if both were nullptr
      if (result_head == nullptr) {
        result_head = new ListNode{list2->val};
        current_node = result_head;
        list2 = list2->next;
      } else {
        current_node->next = new ListNode{list2->val};
        current_node = current_node->next;
        list2 = list2->next;
      }
    } else if (list2 == nullptr) {
      if (result_head == nullptr) {
        result_head = new ListNode{list1->val};
        current_node = result_head;
        list1 = list1->next;
      } else {
        current_node->next = new ListNode{list1->val};
        current_node = current_node->next;
        list1 = list1->next;
      }
    } else if (list1->val < list2->val) {
      if (result_head == nullptr) {
        result_head = new ListNode{list1->val};
        current_node = result_head;
        list1 = list1->next;
      } else {
        current_node->next = new ListNode{list1->val};
        current_node = current_node->next;
        list1 = list1->next;
      }
    } else {
      if (result_head == nullptr) {
        result_head = new ListNode{list2->val};
        current_node = result_head;
        list2 = list2->next;
      } else {
        current_node->next = new ListNode{list2->val};
        current_node = current_node->next;
        list2 = list2->next;
      }
    }
  }

  // Clean UP
  delete list1;
  delete list2;

  return result_head;
}
