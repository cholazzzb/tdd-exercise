"use strict";
// TAGS: Graph Theory
exports.__esModule = true;
exports.validPath = void 0;
function validPath(n, edges, source, destination) {
    if (source === destination)
        return true;
    var graph = Array(n)
        .fill(null)
        .map(function (_) { return ({}); });
    var visited = Array(n).fill(false);
    for (var _i = 0, edges_1 = edges; _i < edges_1.length; _i++) {
        var edge = edges_1[_i];
        graph[edge[0]][edge[1]] = edge[1];
        graph[edge[1]][edge[0]] = edge[0];
    }
    var exist = false;
    var dfs = function (end) {
        if (end === destination) {
            exist = true;
            return;
        }
        if (visited[end])
            return;
        visited[end] = true;
        for (var _i = 0, _a = Object.values(graph[end]); _i < _a.length; _i++) {
            var neighbour = _a[_i];
            dfs(neighbour);
        }
    };
    visited[source] = true;
    for (var _a = 0, _b = Object.values(graph[source]); _a < _b.length; _a++) {
        var neighbour = _b[_a];
        dfs(neighbour);
    }
    return exist;
}
exports.validPath = validPath;
