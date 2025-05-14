import sys
from lib import Idx, Mat 

lines = [line.strip() for line in sys.stdin.readlines()]

def compute_score(b):
    s = 0
    for idx in b.indices():
        if b[idx] == "O":
            s += 100 * idx[0] + idx[1]
    return s


DIRS = {">": Idx(0, 1), "v": Idx(1, 0), "<": Idx(0, -1), "^": Idx(-1, 0)}

def next(idx, d):
    return idx + DIRS[d]

def move_obj(b, idx, d):
    idx1 = next(idx, d)
    if not b.within(idx1):
        return False, idx

    match (b[idx1]):
        case ".":
            moved = True
        case "#":
            moved = False
        case "O":
            moved, _ = move_obj(b, idx1, d)
        case _:
            raise Exception("char not recognized")

    if moved:
        b.swap(idx1, idx)
        idx = idx1
  
    return moved, idx

i = 0
b = []
while lines[i] != "":
    b.append(list(lines[i]))
    i += 1

b = Mat(b)

moves = "".join(lines[i + 1 :])
print(b)
idx = b.find('@')
for move in moves:
    _, idx = move_obj(b, idx, move)

print(compute_score(b))
