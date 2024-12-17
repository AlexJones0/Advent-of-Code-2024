"""
FILE: Day 17/Python/sol.py
Author: Alex Jones
Desc: Solution to day 17 problems (33 & 34) for Advent of Code 2024, solved in Python 3.
"""
data = open("Day 17/data.txt", "r").read().strip().replace("\r","").split("\n\n")
regs = [int(r.split(":")[1].strip()) for r in data[0].split("\n")]
instrs = [int(i) for i in data[1].split(":")[1].split(",")]
A, B, C = 0, 1, 2

def simulate(regs: list[int], instrs: list[int]):
    out, pcc = [], 0
    while pcc < len(instrs):
        opcode, operand = instrs[pcc:pcc+2]
        combo = operand if operand < 4 or operand == 7 else regs[operand-4]
        if opcode == 0: regs[A] = regs[A] // (2 ** combo)    # adv
        elif opcode == 1: regs[B] ^= operand                 # bxl
        elif opcode == 2: regs[B] = combo % 8                # bst
        elif opcode == 3 and regs[A] != 0: pcc = operand     # jnz
        elif opcode == 4: regs[B] ^= regs[C]                 # bxc
        elif opcode == 5: out.append(combo % 8)              # out
        elif opcode == 6: regs[B] = regs[A] // (2 ** combo)  # bdv
        elif opcode == 7: regs[C] = regs[A] // (2 ** combo)  # cdv
        pcc += 2 if opcode != 3 or regs[A] == 0 else 0
    return out

print("Problem 33:", ",".join(str(o) for o in simulate([r for r in regs], instrs)))

def dfs(regs: list[int], instrs: list[int], state: int, instr: int) -> int:
    for i in range(8):
        test_state = state + i
        test_regs = [test_state, regs[B], regs[C]]
        if simulate(test_regs, instrs)[0] == instrs[instr]:
            if instr == 0:
                return test_state
            if (res := dfs(regs, instrs, test_state * 8, instr-1)) != -1:
                return res
    return -1

print("Problem 34:", dfs(regs, instrs, 0, len(instrs)-1))



"""
Problem 34 explanation:

My program, decoded:
0:  bst [A]  :   [B] = [A] % 8
2:  bxl 5    :   [B] = [B] XOR 101
4:  cdv [B]  :   [C] = [A] // (1 << [B])
6:  bxl 6    :   [B] = [B] XOR 110
8:  adv 3    :   [A] = [A] // 8
10: bxc      :   [B] = [B] XOR [C]
12: out [B]  :   output ([B] % 8)
14: JNZ 0    :   jump to 0 if [A] != 0

At each iteration:
  [A] = [A] // 8
  [C] = [A] // (2 ** [B])      <which is just   A // (1 << [B])>
  [B] = [B] XOR 011 XOR [C]
  OUTPUT [B] % 8

So just work backwards:
  Try A = 0..7
  Which ones give the correct last output when computed?
  Then try A = (any prev A) * 8 + 0..7
  Which ones give the correct last 2 outputs when computed?
  ...
And so on until the final output is constructed.
Sometimes a value might seem to work at first, but then lead to no possibilities
later, so at each step we consider all values. But we don't need to compute all
values immediately - only when previous values fail. We effectively do a DFS from
the solution back to the answer, always exploring the smallest values of register
A first to ensure that the first solution we find is the smallest.

"""
