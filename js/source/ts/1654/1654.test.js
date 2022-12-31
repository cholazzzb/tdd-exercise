const { minimumJumps } = require("./1654");

describe("test 1654.js", () => {
  it("1", () => {
    expect(minimumJumps([14, 4, 18, 1, 15], 3, 15, 9)).toBe(3);
  });
  it("2", () => {
    expect(minimumJumps([8, 3, 16, 6, 12, 20], 15, 13, 11)).toBe(-1);
  });
  it("3", () => {
    expect(minimumJumps([1, 6, 2, 14, 5, 17, 4], 16, 9, 7)).toBe(2);
  });
});
