from typing import List

class Solution:
    def removeDuplicates(self, nums: List[int]) -> int:
        unique_nums = []
        for i in nums:
            if i in unique_nums:
                continue
            else:
                nums[len(unique_nums)] = i
                unique_nums.append(i)
        return len(unique_nums)