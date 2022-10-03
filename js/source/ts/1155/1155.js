"use strict";
exports.__esModule = true;
exports.numRollsToTarget = void 0;
var mod = Math.pow(10, 9) + 7;
function numRollsToTarget(n, k, target) {
    if (target > n * k) {
        return 0;
    }
    var dp = Array(n)
        .fill(0)
        .map(function (_) { return []; });
    for (var idx = 0; idx < k; idx++) {
        dp[0][idx] = 1;
    }
    for (var dice = 1; dice < n; dice++) {
        for (var x = 0; x < n * k; x++) {
            if (x < dice) {
                dp[dice][x] = 0;
            }
            else if (x === dice) {
                dp[dice][x] = 1;
            }
            else {
                dp[dice][x] = 0;
                for (var f = 1; f <= k; f++) {
                    if (dp[dice - 1][x - f]) {
                        dp[dice][x] += dp[dice - 1][x - f];
                        dp[dice][x] %= mod;
                    }
                }
            }
        }
    }
    return dp[n - 1][target - 1];
}
exports.numRollsToTarget = numRollsToTarget;
