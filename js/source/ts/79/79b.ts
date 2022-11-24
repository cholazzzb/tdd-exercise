function exist(board: string[][], word: string): boolean {
  for (let r = 0; r < board.length; r++) {
    for (let c = 0; c < board[0].length; c++) {
      if (board[r][c] === word[0] && dfs(r, c, 0, board, word)) return true;
    }
  }
  return false;
}

function dfs(
  r: number,
  c: number,
  index: number,
  board: Array<Array<string>>,
  word: string
): boolean | undefined {
  if (word.length === index) return true;
  if (
    r < 0 ||
    c < 0 ||
    r >= board.length ||
    c >= board[0].length ||
    board[r][c] != word[index]
  ) {
    return false;
  }
  board[r][c] = "#";
  if (
    dfs(r + 1, c, index + 1, board, word) ||
    dfs(r - 1, c, index + 1, board, word) ||
    dfs(r, c + 1, index + 1, board, word) ||
    dfs(r, c - 1, index + 1, board, word)
  ) {
    return true;
  }
  board[r][c] = word[index];
}

export { exist };
