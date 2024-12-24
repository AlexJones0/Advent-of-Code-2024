"""
FILE: Day 24/Python/sol.py
Author: Alex Jones
Desc: Solution to day 24 problems (47 & 48) for Advent of Code 2024, solved in Python 3.
"""
data = open("Day 24/data.txt", "r").read().strip().replace("\r","").split("\n\n")
wires = {w[0]: bool(int(w[1])) for w in [w.split(": ") for w in data[0].split("\n")]}
gates = [(g[1], g[0], g[2], g[4]) for g in [g.split(" ") for g in data[1].split("\n")]]
ops = {"AND": lambda x,y: x and y, "OR": lambda x,y: x or y, "XOR": lambda x, y: x ^ y}

# Simulate gates/circuit
sim_gates = gates.copy()
while sim_gates:
    to_remove = []
    for gate in sim_gates:
        (op, inp1, inp2, wire) = gate
        if inp1 in wires and inp2 in wires:
            wires[wire] = ops[op](wires[inp1], wires[inp2])
            to_remove.append(gate)
    for gate in to_remove:
        sim_gates.remove(gate)

# Collect answer from zXX bits
p1_ans = 0
for gate, val in wires.items():
    if gate[0] == "z":
        p1_ans = p1_ans | (int(val) << int(gate[1:]))
print("Problem 47:", p1_ans)

# Compute the actual result of addition
x, y = 0, 0
for gate, val in wires.items():
    if gate[0] == "x":
        x = x | (int(val) << int(gate[1:]))
    elif gate[0] == "y":
        y = y | (int(val) << int(gate[1:]))
target_ans = x + y

# Compute the MSB of the output
output_msb = 0
for wire in wires:
    if wire[0] == "z":
        output_msb = max(output_msb, int(wire[1:]))
output_msb_wire = "z" + str(output_msb)

# Find the input LSBs
x_lsb, y_lsb = "x0", "y0"
while x_lsb not in wires or y_lsb not in wires:
    x_lsb += "0"
    y_lsb += "0"

# Use the structure of a ripple adder to figure out which gates are wrong, and thus
# need to be swapped. We don't need to actually figure out the pairs for swapping.
needs_swapping = set()
for (op, inp1, inp2, wire) in gates:
    # Check if the final carry is wrong, seems never to happen though
    if wire == output_msb_wire:
        if (p1_ans & output_msb) != (target_ans & output_msb):
            needs_swapping.add(wire)
    # Check if the LSB is wrong, seems never to happen though
    elif inp1 in [x_lsb, y_lsb] or inp2 in [x_lsb, y_lsb] and op == "AND":
        if (p1_ans & 1) != (target_ans & 1):
            needs_swapping.add(wire)
    # XOR is only used for output bits ("zxx") or on inputs. Intermediary gates
    # are all AND or OR, so something needs to swap here
    elif op == "XOR" and all(w[0] not in "xyz" for w in (wire, inp1, inp2)):
        needs_swapping.add(wire)
    # All outputs (minus the MSB) appear to be XOR, so if not something needs swapping
    elif wire[0] == "z" and op != "XOR":
        needs_swapping.add(wire)
    # For the final case, it appears that in the input ripple adder construction, we
    # always have AND->OR, XOR->AND, or XOR->XOR (except for the input LSB where we
    # have a half adder). So, we just detect gate pairings - if we see any AND->AND,
    # AND->XOR, or XOR->OR, we know the current wire (1st output) must be incorrect,
    # and thus needs swapping.
    else:
        for (next_op, next_inp1, next_inp2, _) in gates:
            if (wire == next_inp1 or wire == next_inp2) and \
               (op == "AND" and next_op != "OR" or op == "XOR" and next_op == "OR"):
                needs_swapping.add(wire)
print("Problem 48:", ",".join(sorted(needs_swapping)))
