"""
FILE: Day 08/Python/sol.py
Author: Alex Jones
Desc: Solution to day 8 problems (15 & 16) for Advent of Code 2024, solved in Python 3.
"""
data = open("Day 08/data.txt", "r").read().strip().replace("\r","").split("\n")

def in_bounds(pos, data):
    return 0 <= pos[0] < len(data) and 0 <= pos[1] < len(data[pos[0]])

def find_antinodes(node, dy, dx, count, p1_set, p2_set):
    while in_bounds(node, data):
        if (count := count + 1) == 1:
            p1_set.add(node)
        p2_set.add(node)
        node = (node[0] + dy, node[1] + dx)

from collections import defaultdict
node_frequencies = defaultdict(list)
for i, row in enumerate(data):
    for j, node in enumerate(row):
        if node != '.':
            node_frequencies[node].append((i, j))

antinodes_p1, antinodes_p2 = set(), set()
for nodes in node_frequencies.values():
    for i, node1 in enumerate(nodes):
        for node2 in nodes[(i+1):]:
            dy, dx = (node2[0] - node1[0], node2[1] - node1[1])
            find_antinodes(node2, dy, dx, -1, antinodes_p1, antinodes_p2)
            find_antinodes(node1, -dy, -dx, -1, antinodes_p1, antinodes_p2)

print("Problem 15:", len(antinodes_p1))
print("Problem 16:", len(antinodes_p2))
