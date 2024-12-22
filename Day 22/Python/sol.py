"""
FILE: Day 22/Python/sol.py
Author: Alex Jones
Desc: Solution to day 22 problems (43 & 44) for Advent of Code 2024, solved in Python 3.
"""
data = [int(n) for n in open("Day 22/data.txt", "r").read().strip().replace("\r","").split("\n")]

from collections import deque

def get_next_secret_number(num: int) -> int:
    num = ((num << 6 ) ^ num) & 0xFFFFFF
    num = ((num >> 5 ) ^ num) & 0xFFFFFF
    num = ((num << 11) ^ num) & 0xFFFFFF
    return num

bananas = [0 for _ in range(130321)]
deltas = deque(maxlen=4)
for i in range(len(data)):
    price = data[i] % 10
    seen = set()
    for _ in range(3):
        data[i] = get_next_secret_number(data[i])
        new_price = data[i] % 10
        deltas.append(new_price - price)
        price = new_price
    for _ in range(1997):
        data[i] = get_next_secret_number(data[i])
        new_price = data[i] % 10
        deltas.append(new_price - price)
        price = new_price
        perfect_hash = 61731 + deltas[0] * 6859 + deltas[1] * 361 + deltas[2] * 19 + deltas[3]
        if perfect_hash not in seen:
            seen.add(perfect_hash)
            bananas[perfect_hash] += price
print("Problem 43:", sum(data))
print("Problem 44:", max(bananas))