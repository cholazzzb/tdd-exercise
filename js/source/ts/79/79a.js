"use strict";
var __spreadArray = (this && this.__spreadArray) || function (to, from, pack) {
    if (pack || arguments.length === 2) for (var i = 0, l = from.length, ar; i < l; i++) {
        if (ar || !(i in from)) {
            if (!ar) ar = Array.prototype.slice.call(from, 0, i);
            ar[i] = from[i];
        }
    }
    return to.concat(ar || Array.prototype.slice.call(from));
};
exports.__esModule = true;
exports.exist = void 0;
var dirs = [[0, 1], [0, -1], [1, 0], [-1, 0]];
function exist(board, word) {
    var check = function (y, x, mem, lBoard) {
        if (board[y][x] !== word[0]) {
            return false;
        }
        var qPos = [[y, x]];
        var qMem = [mem];
        var qBoard = [lBoard];
        while (qPos.length > 0) {
            var _a = qPos.pop(), y_1 = _a[0], x_1 = _a[1];
            var mem_1 = qMem.pop();
            var board_1 = qBoard.pop();
            if (board_1[y_1][x_1] === word[mem_1.length]) {
                board_1[y_1][x_1] = "-";
                if (mem_1 + word[mem_1.length] === word) {
                    return true;
                }
            }
            for (var _i = 0, dirs_1 = dirs; _i < dirs_1.length; _i++) {
                var _b = dirs_1[_i], dy = _b[0], dx = _b[1];
                var ny = y_1 + dy;
                var nx = x_1 + dx;
                if (ny >= 0 && ny < board_1.length && nx >= 0 && nx < board_1[0].length) {
                    if (board_1[ny][nx] !== "-") {
                        if (board_1[ny][nx] === word[mem_1.length + 1]) {
                            qPos.push([ny, nx]);
                            qMem.push(mem_1 + word[mem_1.length]);
                            qBoard.push(__spreadArray([], board_1.map(function (row) { return __spreadArray([], row, true); }), true));
                        }
                    }
                }
            }
        }
        return false;
    };
    for (var y = 0; y < board.length; y++) {
        for (var x = 0; x < board[0].length; x++) {
            if (check(y, x, "", __spreadArray([], board.map(function (row) { return __spreadArray([], row, true); }), true))) {
                return true;
            }
        }
    }
    return false;
}
exports.exist = exist;
;
