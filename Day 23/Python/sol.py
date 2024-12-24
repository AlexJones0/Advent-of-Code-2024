"""
FILE: Day 23/Python/sol.py
Author: Alex Jones
Desc: Solution to day 23 problems (45 & 46) for Advent of Code 2024, solved in Python 3.
"""
data = [r.split("-") for r in open("Day 23/data.txt", "r").read().strip().replace("\r","").split("\n")]

from collections import defaultdict

graph = defaultdict(set)
for (fst, snd) in data:
    graph[fst].add(snd)
    graph[snd].add(fst)
target_cliques = set()
for node1 in [n for n in graph.keys() if n[0] == 't']:
    for node2 in graph[node1]:
        for node3 in graph[node1]:
            if node3 in graph[node2]:
                target_cliques.add(tuple(sorted((node1, node2, node3))))
print("Problem 45:", len(target_cliques))

# The input graph is simple enough that a greedy / brute force approach will
# work. For each node start by considering the full set of neighbours. By
# assuming that the maximum clique is quite close to the maximum degree (it
# is, both appear to be around 13/14), we can just iteratively try removing
# all nodes from our initial sets of all nodes connected to one node, until
# we find a clique. By processing in a FIFO queue, the first clique we
# find is necessarily maximal. We could also use a heap where the cost is
# the size, but it seems not needed here.
queue = [sorted(list(ngbs) + [node]) for node, ngbs in graph.items()]
p2_ans = None
while queue:
    maybe_clique = queue.pop(0)
    if all(maybe_clique[j] in graph[n1] for i, n1 in enumerate(maybe_clique)
                                        for j in range(i)):
        p2_ans = ",".join(maybe_clique)
        break
    queue += [[n for n in maybe_clique if n != ngb] for ngb in maybe_clique]
print("Problem 46:", p2_ans)