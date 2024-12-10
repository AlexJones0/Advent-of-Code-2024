"""
FILE: Day 09/Python/sol.py
Author: Alex Jones
Desc: Solution to day 9 problems (17 & 18) for Advent of Code 2024, solved in Python 3.
"""
data = [int(n) for n in open("Day 09/data.txt", "r").read().strip()]

# O(n) Two pointer solution using sizes
files, gaps, moved = data[2::2], data[1::2], [0] * data[0]

gap_index, file_index = 0, len(files) - 1
while gap_index < file_index:
    size = files[file_index]
    if size <= gaps[gap_index]:
        moved += [file_index+1] * size
        file_index -= 1
        gaps[gap_index] -= size
    else:
        moved += [file_index+1] * gaps[gap_index]
        files[file_index] = size - gaps[gap_index]
        gaps[gap_index] = 0
    if gaps[gap_index] == 0:
        moved += [gap_index+1] * files[gap_index]
        gap_index += 1
if gap_index == file_index:
    moved += [gap_index+1] * files[gap_index]
print("Problem 17:", sum(i * val for i, val in enumerate(moved)))

#O(n log n) solution using an array map for 1-9) of priority queues
import heapq
files, gaps = [], [[] for _ in range(10)] 

position = 0
for index, size in enumerate(data):
    if index % 2 == 0:
        files.append((position, size))
    elif size > 0:
        heapq.heappush(gaps[size], position)
    position += size

for value, (file_pos, file_size), in list(enumerate(files))[::-1]:
    (gap_pos, gap_size) = min([(gaps[size][0], size) for size in range(file_size, 10)], default=(0,0))
    if gap_pos and file_pos > gap_pos:
        files[value] = (gap_pos, file_size)
        heapq.heappop(gaps[gap_size])
        heapq.heappush(gaps[gap_size - file_size], gap_pos + file_size)

print("Problem 18:", sum(sum(range(start,start+size)) * value for value, (start, size) in enumerate(files)))
