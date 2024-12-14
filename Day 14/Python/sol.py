"""
FILE: Day 14/Python/sol.py
Author: Alex Jones
Desc: Solution to day 14 problems (27 & 28) for Advent of Code 2024, solved in Python 3.
"""
data = open("Day 14/data.txt", "r").read().strip().replace("\r","").split("\n")
data = [tuple(tuple(int(val.strip()) for val in item.split("=")[-1].split(","))
         for item in line.split())
         for line in data]
SPACE_WIDTH, SPACE_HEIGHT = 101, 103

def simulate_robots(robots, timesteps):
    t = -1
    while (t := t + 1) != timesteps:
        for i, (pos, vel) in enumerate(robots):
            new_pos = ((pos[0] + vel[0]) % SPACE_WIDTH, 
                       (pos[1] + vel[1]) % SPACE_HEIGHT)
            robots[i] = (new_pos, vel)
        # From intuition - target picture (Christmas tree) has the highest set
        # of unique positions. After experimentation, it turns out to be the
        # only state that has every robot position unique!
        if timesteps == -1 and len(set(r[0] for r in robots)) == len(robots):
            return t+1


def get_safety_factor(robots):
    quad = [0, 0, 0, 0]
    mid_x, mid_y = SPACE_WIDTH // 2, SPACE_HEIGHT // 2
    for ((x, y), _) in robots:
        if x == mid_x or y == mid_y:
            continue
        quad[2 * (x < mid_x) + (y < mid_y)] += 1
    return quad[0] * quad[1] * quad[2] * quad[3]

simulate_robots(data, 100)
print("Problem 27:", get_safety_factor(data))
print("Problem 28:", simulate_robots(data, -1) + 100)
