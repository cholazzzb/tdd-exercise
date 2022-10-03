const { minCost } = require("./1578");

describe("test 1578.js", () => {
  it("1", () => {
    expect(minCost("abaac", [1, 2, 3, 4, 5])).toBe(3);
  });
  it("2", () => {
    expect(minCost("abc", [1, 2, 3])).toBe(0);
  });
  it("3", () => {
    expect(minCost("aabaa", [1, 2, 3, 4, 1])).toBe(2);
  });
  it("4", () => {
    expect(minCost("aaabbbabbbb", [3, 5, 10, 7, 5, 3, 5, 5, 4, 8, 1])).toBe(26);
  });
});
