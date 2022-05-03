const findUnsortedSubarray = require("./581");

describe("test 581.js", () => {
  it("1", () => {
    expect(findUnsortedSubarray([2, 6, 4, 8, 10, 9, 15])).toBe(5);
  });
  it("2", () => {
    expect(findUnsortedSubarray([1, 2, 3, 4])).toBe(0);
  });
  it("3", () => {
    expect(findUnsortedSubarray([1, 3, 2, 2, 2])).toBe(4);
  });
  it("4", () => {
    expect(findUnsortedSubarray([1])).toBe(0);
  });
  it("5", () => {
    expect(findUnsortedSubarray([5, 4, 3, 2, 1])).toBe(5);
  });
  it("6", () => {
    expect(findUnsortedSubarray([1, 3, 2, 3, 3])).toBe(2);
  });
  it("7", () => {
    expect(findUnsortedSubarray([1, 2, 4, 5, 3])).toBe(3);
  });
});
