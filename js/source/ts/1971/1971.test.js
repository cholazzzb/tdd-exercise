const { validPath } = require("./1971");

describe("test 1971.js", () => {
  it("1", () => {
    const n = 3;
    const edges = [
      [0, 1],
      [1, 2],
      [2, 0],
    ];
    const source = 0;
    const destination = 2;

    const expected = true;
    expect(validPath(n, edges, source, destination)).toBe(expected);
  });
  it("2", () => {
    const n = 6;
    const edges = [
      [0, 1],
      [0, 2],
      [3, 5],
      [5, 4],
      [4, 3],
    ];
    const source = 0;
    const destination = 5;

    const expected = false;
    expect(validPath(n, edges, source, destination)).toBe(expected);
  });
  it("3", () => {
    const n = 10;
    const edges = [
      [0, 7],
      [0, 8],
      [6, 1],
      [2, 0],
      [0, 4],
      [5, 8],
      [4, 7],
      [1, 3],
      [3, 5],
      [6, 5],
    ];
    const source = 7;
    const destination = 5;

    const expected = true;
    expect(validPath(n, edges, source, destination)).toBe(expected);
  });
});
