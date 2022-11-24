"use strict";
exports.__esModule = true;
exports.numSquares = void 0;
function numSquares(n) {
    var dp = Array(n + 1).fill(n);
    dp[1] = 1;
    for (var idx = 1; idx < n + 1; idx++) {
        var root = Math.sqrt(idx);
        if (root % 1 === 0) {
            dp[idx] = 1;
        }
        else {
            var j = 1;
            var power = j * j;
            while (power <= idx) {
                dp[idx] = Math.min(dp[idx], dp[power] + dp[idx - power]);
                j += 1;
                power = j * j;
            }
        }
    }
    return dp[n];
}
exports.numSquares = numSquares;
