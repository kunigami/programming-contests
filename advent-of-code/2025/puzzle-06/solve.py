is_hard = True
import sys


def read_input1():
    lines = sys.stdin.readlines()

    raw = []
    for i in range(len(lines)-1):
        raw.append([int(x) for x in lines[i].split()])
    # tranpose
    values = []
    for j in range(len(raw[0])):
        values.append([raw[i][j] for i in range(len(raw))])

    ops = [str(x) for x in lines[-1].split()]

    return values, ops

def read_input2():
    lines = sys.stdin.read().rstrip().split('\n')

    values = []
    vs = []
    L = max(len(l) for l in lines)

    for j in range(L):
        s = ''
        for i in range(len(lines)-1):
            if j >= len(lines[i]):
                continue
            s += lines[i][j]

        if s.strip() == '':
            values.append(vs)
            vs = []
        else:
            vs.append(int(s))
    values.append(vs)

    ops = [str(x) for x in lines[-1].split()]

    return values, ops

def solve(values, ops):
    t = 0
    for i in range(len(values)):
        op = ops[i]
        s = 0 if op == '+' else 1

        for j in range(len(values[i])):
            v = values[i][j]
            s = s + v if op == '+' else s * v
        t += s
    return t

def solve_easy():
    values, ops = read_input1()
    return solve(values, ops)

def solve_hard():
    values, ops = read_input2()
    return solve(values, ops)


print(solve_hard() if is_hard else solve_easy())
