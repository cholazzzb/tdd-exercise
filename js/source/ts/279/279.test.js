const { numSquares } = require("./279");

describe("test 279.js", () => {
  it("1", () => {
    expect(numSquares(1)).toBe(1);
  });
  it("2", () => {
    expect(numSquares(2)).toBe(2);
  });
  it("3", () => {
    expect(numSquares(3)).toBe(3);
  });
  it("4", () => {
    expect(numSquares(4)).toBe(1);
  });
  it("5", () => {
    expect(numSquares(5)).toBe(2);
  });
  it("6", () => {
    expect(numSquares(6)).toBe(3);
  });
  it("7", () => {
    expect(numSquares(7)).toBe(4);
  });
  it("8", () => {
    expect(numSquares(8)).toBe(2);
  });
  it("9", () => {
    expect(numSquares(9)).toBe(1);
  });
  it("10", () => {
    expect(numSquares(10)).toBe(2);
  });
  it("11", () => {
    expect(numSquares(11)).toBe(3);
  });
  it("12", () => {
    expect(numSquares(12)).toBe(3);
  });
  it("13", () => {
    expect(numSquares(13)).toBe(2);
  });
});
