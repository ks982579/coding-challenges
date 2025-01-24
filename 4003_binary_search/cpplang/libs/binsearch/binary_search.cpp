#include "binary_search.h"
#include <iostream>

// Assuming the vector is organized.
int Solution::search(std::vector<int> &nums, int target) {
  std::cout << "Running " << std::endl;
  // should trunc automatically
  int check(nums.size() / 2);
  std::cout << check << std::endl;
  int left = 0, right = nums.size() - 1;

  while (left <= right) {
    int mid = left + (right - left) / 2;
    if (nums.at(mid) == target)
      return mid;
    if (nums.at(mid) < target) {
      left = mid + 1;
    } else {
      right = mid - 1;
    }
  }

  // -----
  return -1;
};
