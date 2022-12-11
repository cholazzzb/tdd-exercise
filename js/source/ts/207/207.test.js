const { canFinish } = require("./207");

describe("test 207.js", () => {
  it("1", () => {
    const numCourses = 2;
    const prerequisites = [[1, 0]];

    const expected = true;
    expect(canFinish(numCourses, prerequisites)).toBe(expected);
  });
  it("2", () => {
    const numCourses = 2;
    const prerequisites = [
      [1, 0],
      [0, 1],
    ];

    const expected = false;
    expect(canFinish(numCourses, prerequisites)).toBe(expected);
  });
  it("3", () => {
    const numCourses = 5;
    const prerequisites = [
      [1, 4],
      [2, 4],
      [3, 1],
      [3, 2],
    ];

    const expected = true;
    expect(canFinish(numCourses, prerequisites)).toBe(expected);
  });
});
