"""
FILE: Day 13/Python/sol.py
Author: Alex Jones
Desc: Solution to day 13 problems (25 & 26) for Advent of Code 2024, solved in Python 3.
"""
data = open("Day 13/data.txt", "r").read().strip().replace("\r","").split("\n\n")

"""There is actually only 1 way to solve each - use simultaneous eqs.
[1] ax * A + bx * B = gx
[2] ay * A + by * B = gy
Re-arrange [2]: A = (gy - by * B)/ay
Sub in [1] to get [3]: (ax * (gy - by * B)) / ay + bx * B = gx
Re-arrange [3] to get [4]: (ax * gy)/ay - (ax * by * B))/ay + bx * B = gx
                            bx * B - (ax * by * B)/ay = gx - (ax * gy)/ay
                            B(bx - (ax * by)/ay) = gx - (ax * gy)/ay
                            B = (gx - (ax * gy)/ay)/(bx - (ax * by)/ay)
                            B = (gx * ay - ax * gy)/(ay * (bx - (ax * by)/ay))
                            B = (gx * ay - ax * gy)//(ay * bx - ax * by)
 Use [4] and then [2] to solve for B and A."""
def least_moves(moves: list[tuple[int,int]], goal: tuple[int,int]) -> int:
    (ax, ay), (bx, by), (gx, gy) = moves[0], moves[1], goal
    numerator, denom = gx * ay - ax * gy, ay * bx - ax * by
    if denom == 0: return 0  # Impossible to solve
    elif numerator % denom != 0: return 0  # Non-integer, impossible in context
    B = numerator // denom
    numerator, denom = gy - by * B, ay
    if numerator % denom != 0: return 0  # Non-integer, impossible in context
    A = numerator // denom
    return A * 3 + B
    

p1, p2 = 0, 0
for entry in data:
    entry = [row.split(":")[-1].split(",") for row in entry.split("\n")]
    entry = [[int(d.replace("=","+").split("+")[-1].strip()) for d in row]
             for row in entry]
    p1 += least_moves(entry[:-1], entry[-1])
    p2 += least_moves(entry[:-1], [v+10000000000000 for v in entry[-1]])

print("Problem 25:", p1)
print("Problem 26:", p2)
