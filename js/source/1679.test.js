const maxOperations = require("./1679");

describe("test 1679.js", () => {
  it("1", () => {
    expect(maxOperations([1, 2, 3, 4], 5)).toBe(2);
  });
  it("2", () => {
    expect(maxOperations([3, 1, 3, 4, 3], 6)).toBe(1);
  });
  it("3", () => {
    expect(maxOperations([2, 2, 1, 1], 3)).toBe(2);
  });
  it("4", () => {
    expect(
      maxOperations(
        [2, 5, 4, 4, 1, 3, 4, 4, 1, 4, 4, 1, 2, 1, 2, 2, 3, 2, 4, 2],
        3
      )
    ).toBe(4);
  });
});
