# Two Sum

```yaml
url: https://leetcode.com/problems/two-sum/description/
title: Two Sum
```

Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

You can return the answer in any order.

---

I think the most obvious solution would be nested looping.
It has space complexity of $O(1)$ because space requirements would not increates.
However, the time complexity would be $O(n^2)$ because of the nested looping.

We can trade time complexity for space complexity if we use a _hash table_.
This solution is a bit less obvious, but because a hash table has $O(1)$ for lookup cost, if you build the correct hash table, you only loop through the array once.

Using a _Hash Table_ may require a subtle adjustment if you want to allow for negative values as well.
You may need to incorporate information like the index value as well if you want to allow for duplicates since a hash is meant to be unique.
Let's not embark on that path for this challenge though.
