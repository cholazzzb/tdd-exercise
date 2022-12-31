"use strict";
exports.__esModule = true;
exports.minimumJumps = void 0;
function generateHash(node) {
    return node.index * (node.isBackJump ? -1 : 1);
}
function minimumJumps(forbidden, forwardSteps, backwardSteps, endIndex) {
    var forbiddenIndexes = new Set(forbidden);
    var visitedNodes = new Set();
    var outOfBoundsIndex = Math.min(forwardSteps * backwardSteps + endIndex, 10000);
    var startNode = {
        index: 0,
        isBackJump: false,
        jumps: 0
    };
    var queue = new Array(startNode);
    var _loop_1 = function () {
        var _a = queue.shift(), index = _a.index, isBackJump = _a.isBackJump, jumps = _a.jumps;
        // Reached the end
        if (index === endIndex) {
            return { value: jumps };
        }
        var nextNodes = [
            {
                index: index + forwardSteps,
                isBackJump: false,
                jumps: jumps + 1
            },
            {
                index: index - backwardSteps,
                isBackJump: true,
                jumps: jumps + 1
            },
        ];
        nextNodes
            .filter(function (node) {
            return (!forbiddenIndexes.has(node.index) &&
                !visitedNodes.has(generateHash(node)) &&
                !(isBackJump && node.isBackJump) &&
                node.index >= 0 &&
                node.index <= outOfBoundsIndex);
        })
            .forEach(function (node) {
            visitedNodes.add(generateHash(node));
            queue.push(node);
        });
    };
    while (queue.length > 0) {
        var state_1 = _loop_1();
        if (typeof state_1 === "object")
            return state_1.value;
    }
    return -1;
}
exports.minimumJumps = minimumJumps;
