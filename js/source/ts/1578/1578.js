"use strict";
exports.__esModule = true;
exports.minCost = void 0;
function minCost(colors, neededTime) {
    var res = 0;
    var initColor = colors[0];
    var max = neededTime[0];
    var dif = 0;
    for (var idx = 1; idx < colors.length; idx++) {
        var curColor = colors[idx];
        if (initColor !== curColor) {
            initColor = curColor;
            dif = 0;
            continue;
        }
        if (dif === 0) {
            if (neededTime[idx - 1] < neededTime[idx]) {
                max = neededTime[idx];
                res += neededTime[idx - 1];
            }
            else {
                max = neededTime[idx - 1];
                res += neededTime[idx];
            }
        }
        else {
            if (neededTime[idx] > max) {
                res += max;
                max = neededTime[idx];
            }
            else {
                res += neededTime[idx];
            }
        }
        dif++;
    }
    return res;
}
exports.minCost = minCost;
