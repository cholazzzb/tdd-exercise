/**
 * @param {number[]} nums
 * @return {number}
 */
var findUnsortedSubarray = function (nums) {
  let left = Infinity,
    right = -Infinity;
  let minPrev = nums[nums.length - 1],
    maxPrev = nums[0];

  for (let i = 1, j = nums.length - 2; i < nums.length; i++, j--) {
    const iNum = nums[i];
    const jNum = nums[j];

    if (iNum < maxPrev) {
      right = i;
    }
    maxPrev = Math.max(maxPrev, iNum);
    if (jNum > minPrev) {
      left = j;
    }
    minPrev = Math.min(minPrev, jNum);
  }

  return left < right ? right - left + 1 : 0;
};

module.exports = findUnsortedSubarray;
