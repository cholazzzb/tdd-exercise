const { numRollsToTarget } = require("./1155");

describe("test 1155.js", () => {
  it("1", () => {
    expect(numRollsToTarget(1, 6, 3)).toBe(1);
  });
  it("2", () => {
    expect(numRollsToTarget(8, 6, 3)).toBe(0);
  });
  it("3", () => {
    expect(numRollsToTarget(2, 6, 7)).toBe(6);
  });
  it("4", () => {
    expect(numRollsToTarget(5, 6, 15)).toBe(651);
  });
  it("5", () => {
    expect(numRollsToTarget(30, 30, 500)).toBe(222616187);
  });
});
