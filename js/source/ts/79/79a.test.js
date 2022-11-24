const { exist } = require("./79a");

describe("test 79a.js", () => {
  it("1", () => {
    expect(
      exist(
        [
          ["A", "B", "C", "E"],
          ["S", "F", "C", "S"],
          ["A", "D", "E", "E"],
        ],
        "ABCCED"
      )
    ).toBe(true);
  });
  it("2", () => {
    expect(
      exist(
        [
          ["A", "B", "C", "E"],
          ["S", "F", "C", "S"],
          ["A", "D", "E", "E"],
        ],
        "SEE"
      )
    ).toBe(true);
  });
  it("3", () => {
    expect(
      exist(
        [
          ["A", "B", "C", "E"],
          ["S", "F", "C", "S"],
          ["A", "D", "E", "E"],
        ],
        "ABCB"
      )
    ).toBe(false);
  });
  it("4", () => {
    expect(
      exist(
        [
          ["A", "B", "C", "E"],
          ["S", "F", "E", "S"],
          ["A", "D", "E", "E"],
        ],
        "ABCESEEEFS"
      )
    ).toBe(true);
  });
  it("5", () => {
    expect(
      exist(
        [
          ["A", "A", "A", "A", "A", "A"],
          ["A", "A", "A", "A", "A", "A"],
          ["A", "A", "A", "A", "A", "A"],
          ["A", "A", "A", "A", "A", "A"],
          ["A", "A", "A", "A", "A", "A"],
          ["A", "A", "A", "A", "A", "A"],
        ],
        "BAAAAAAAAAAAAAA"
      )
    ).toBe(false);
  });
});
