const { findOrder } = require("./210");

describe("test 210.js", () => {
  it("1", () => {
    const numCourses = 2;
    const prerequisites = [[1, 0]];

    const expected = [0, 1];
    const result = findOrder(numCourses, prerequisites);
    result.forEach((res, idx) => {
      expect(res).toBe(expected[idx]);
    });
  });
  it("2", () => {
    const numCourses = 4;
    const prerequisites = [
      [1, 0],
      [2, 0],
      [3, 1],
      [3, 2],
    ];

    const expected = [0, 1, 2, 3];
    const result = findOrder(numCourses, prerequisites);
    result.forEach((res, idx) => {
      expect(res).toBe(expected[idx]);
    });
  });
  it("3", () => {
    const numCourses = 1;
    const prerequisites = [];

    const expected = [0];
    const result = findOrder(numCourses, prerequisites);
    result.forEach((res, idx) => {
      expect(res).toBe(expected[idx]);
    });
  });
});
