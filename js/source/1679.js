/**
 * @param {number[]} nums
 * @param {number} k
 * @return {number}
 */
var maxOperations = function (nums, k) {
  const map = new Map();
  let res = 0;
  nums.forEach((num) => {
    const val = map.get(num);
    if (val > 0) {
      map.set(num, val - 1);
      res += 1;
    } else {
      if (map.has(k - num)) {
        map.set(k - num, map.get(k - num) + 1);
      } else {
        map.set(k - num, 1);
      }
    }
  });
  return res;
};

module.exports = maxOperations;
