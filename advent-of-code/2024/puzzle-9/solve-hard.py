import sys

digit_string = sys.stdin.read().strip()

digit_list = [int(digit) for digit in digit_string]
n = len(digit_list)

result = [
    (digit, -1) if index % 2 != 0 else (digit, index // 2)
    for index, digit in enumerate(digit_list)
]

# print(result)


def render(result):
    s = ""
    for sz, lbl in result:
        if lbl == -1:
            s += "." * sz
        else:
            s += str(lbl) * sz
    print(s)

def calc(result):
    s = 0
    p = 0
    for sz, lbl in result:
        if lbl != -1:
            for i in range(sz):
                s += (p + i) * lbl
        p += sz
    return s        

temp = result.copy()
i = n - 1
for i in reversed(range(n)):
    sz, lbl = result[i]
    if lbl == -1:
        continue

    moved = False

    new_result = []
    for rj in temp:
        if rj[1] == lbl:
            if moved:
                new_result.append((sz, -1))
            else:
                moved = True
                new_result.append(rj)

            continue

        if rj[1] != -1: 
            new_result.append(rj)
        else:
            if rj[0] >= sz and not moved:
                new_result.append((sz, lbl))
                moved = True
                diff = rj[0] - sz
                if diff > 0:
                    new_result.append((diff, -1))
            else:
                new_result.append(rj)

    temp = new_result
    # print(temp)
# render(temp)
print(calc(temp))
