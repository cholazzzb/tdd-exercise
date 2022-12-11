const { canFinish } = require("./207");

describe("test 1971.js", () => {
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
});
