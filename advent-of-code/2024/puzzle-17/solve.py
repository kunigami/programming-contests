from dataclasses import dataclass


@dataclass
class Regs:
    A: int
    B: int
    C: int


regs = Regs(66752888, 0, 0)

insts = [2, 4, 1, 7, 7, 5, 1, 7, 0, 3, 4, 1, 5, 5, 3, 0]

out = []

def combo(v):
    if v <= 3:
        return v
    if v == 4:
        return regs.A
    if v == 5:
        return regs.B
    if v == 6:
        return regs.C
    raise Exception("cannot happen")


def process(op, v):
    np = None
    match op:
        case 0:
            regs.A = regs.A // (2 ** combo(v))
        case 1:
            regs.B = regs.B ^ v
        case 2:
            regs.B = combo(v) % 8
        case 3:
            if regs.A > 0:
                np = v
        case 4:
            regs.B = regs.B ^ regs.C
        case 5:
            out.append(combo(v) % 8)
        case 6:
            regs.B = regs.A // (2 ** combo(v))
        case 7:
            regs.C = regs.A // (2 ** combo(v))
    return np

p = 0
while p < len(insts):
    np = process(insts[p], insts[p + 1])
    if np is None:
        p += 2
    else:
        p = np
print(out)
