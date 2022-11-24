"use strict";
exports.__esModule = true;
exports.exist = void 0;
function exist(board, word) {
    for (var r = 0; r < board.length; r++) {
        for (var c = 0; c < board[0].length; c++) {
            if (board[r][c] === word[0] && dfs(r, c, 0, board, word))
                return true;
        }
    }
    return false;
}
exports.exist = exist;
function dfs(r, c, index, board, word) {
    if (word.length === index)
        return true;
    if (r < 0 ||
        c < 0 ||
        r >= board.length ||
        c >= board[0].length ||
        board[r][c] != word[index]) {
        return false;
    }
    board[r][c] = "#";
    if (dfs(r + 1, c, index + 1, board, word) ||
        dfs(r - 1, c, index + 1, board, word) ||
        dfs(r, c + 1, index + 1, board, word) ||
        dfs(r, c - 1, index + 1, board, word)) {
        return true;
    }
    board[r][c] = word[index];
}
