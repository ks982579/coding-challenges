# Remove Duplicates from Sorted Array

```yaml
url: https://leetcode.com/problems/remove-duplicates-from-sorted-array/description/
title: Remove Duplicates from Sorted Array
difficulty: easy
```

Given an integer array nums sorted in non-decreasing order, remove the duplicates in-place such that each unique element appears only once. The relative order of the elements should be kept the same. Then return the number of unique elements in nums.

Consider the number of unique elements of nums to be k, to get accepted, you need to do the following things:

Change the array nums such that the first k elements of nums contain the unique elements in the order they were present in nums initially. The remaining elements of nums are not important as well as the size of nums.
Return k.

---

I wasn't going to include this one until I realized that using a `set()` in Python wasn't the best way. 
Sets in Python are implemented using a hash table, which doesn't guarantee any specific order for elements. 
