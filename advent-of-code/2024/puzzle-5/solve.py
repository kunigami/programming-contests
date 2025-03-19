import functools
import sys


def custom_sort(row, orders):
    def cmp(a, b):
        if f"{a}|{b}" in orders:
            return -1
        if f"{b}|{a}" in orders:
            return 1
        return 0

    return sorted(row, key=functools.cmp_to_key(cmp))


input_data = [row.strip() for row in sys.stdin.readlines()]

orders = set()


i = 0
while True:
    row = input_data[i]
    if row == "":
        break
    orders.add(row)
    i += 1

i += 1
t = 0
while i < len(input_data):
    err = False
    row = input_data[i].split(",")
    for j in range(len(row)):
        for k in range(j + 1, len(row)):
            if f"{row[k]}|{row[j]}" in orders:
                err = True
                break
    if err:
        row = custom_sort(row, orders)
        print(row)
        t += int(row[len(row) // 2])

    i += 1
print(t)
