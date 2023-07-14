"""
Solution to Two Sum challenge. 
"""

class Solution(object):
    def twoSum(self, nums, target):
        """
        :type nums: List[int]
        :type target: int
        :rtype: List[int]
        """
        for i in range(len(nums)):
            for j in range(len(nums)):
                if i == j:
                    continue
                elif (nums[i] + nums[j]) == target:
                    return i, j


if __name__ == '__main__':
    test_one = [6,10,13,4]
    target = 13 + 4
    sol = Solution()

    result = sol.twoSum(test_one, target)
    print(result)

