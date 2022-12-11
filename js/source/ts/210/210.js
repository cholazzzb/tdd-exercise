"use strict";
exports.__esModule = true;
exports.findOrder = void 0;
var TopologicalSort = /** @class */ (function () {
    function TopologicalSort(graph, indegree) {
        this.orders = [];
        this.isTopologicalSorted = false;
        this.size = graph.length;
        var count = 0;
        var queue = [];
        for (var idx = 0; idx < this.size; idx++) {
            if (indegree[idx] === 0)
                queue.push(idx);
        }
        while (queue.length > 0) {
            var el = queue.shift();
            this.orders.push(el);
            for (var _i = 0, _a = graph[el]; _i < _a.length; _i++) {
                var vertice = _a[_i];
                indegree[vertice]--;
                if (indegree[vertice] == 0) {
                    queue.push(vertice);
                }
            }
            count++;
        }
        this.isTopologicalSorted = count === this.size;
        if (!this.isTopologicalSorted)
            this.orders = [];
    }
    return TopologicalSort;
}());
function findOrder(numCourses, prerequisites) {
    // define the graph outdegree
    var g = Array(numCourses)
        .fill(null)
        .map(function (_) { return []; });
    // define count of indegree
    var indegree = Array(numCourses).fill(0);
    // build the graph
    for (var _i = 0, prerequisites_1 = prerequisites; _i < prerequisites_1.length; _i++) {
        var p = prerequisites_1[_i];
        // we have to take p[1] in order to take p[0]
        g[p[1]].push(p[0]);
        // increase indegree by 1 for p[0]
        indegree[p[0]]++;
    }
    // init topological sort
    var ts = new TopologicalSort(g, indegree);
    // we can finish all courses only if we can topologically sort
    return ts.orders;
}
exports.findOrder = findOrder;
