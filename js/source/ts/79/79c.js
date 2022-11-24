var exist = function (board, word) {
    if (board.length === 0)
        return false;
    var h = board.length;
    var w = board[0].length;
    var dirs = [
        [-1, 0],
        [0, 1],
        [1, 0],
        [0, -1],
    ];
    var go = function (x, y, k) {
        if (board[x][y] !== word[k])
            return false;
        if (k === word.length - 1)
            return true;
        board[x][y] = "*"; // mark as visited
        for (var _i = 0, dirs_1 = dirs; _i < dirs_1.length; _i++) {
            var _a = dirs_1[_i], dx = _a[0], dy = _a[1];
            var i = x + dx;
            var j = y + dy;
            if (i >= 0 && i < h && j >= 0 && j < w) {
                if (go(i, j, k + 1))
                    return true;
            }
        }
        board[x][y] = word[k]; // reset
        return false;
    };
    for (var i = 0; i < h; i++) {
        for (var j = 0; j < w; j++) {
            if (go(i, j, 0))
                return true;
        }
    }
    return false;
};
