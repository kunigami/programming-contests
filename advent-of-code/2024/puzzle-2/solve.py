import sys


def sign(value):
    if value == 0:
        return 0
    elif value > 0:
        return 1
    else:
        return -1


def is_safe(row):
    s = 0
    for i in range(1, len(row)):
        diff = row[i] - row[i - 1]
        if abs(diff) > 3 or diff == 0:
            return False
        if s == 0:
            s = sign(diff)
        elif s != sign(diff):
            return False
    return True


def is_safe_with_tolerance(row):
    if is_safe(row):
        return True
    for i in range(len(row)):
        if is_safe(row[0:i] + row[i + 1 :]):
            return True
    return False


def solve(matrix):
    return sum([is_safe(row) for row in matrix])


def solve2(matrix):
    return sum([is_safe_with_tolerance(row) for row in matrix])


input_data = sys.stdin.readlines()
# read lines into a matrix of integers
matrix = []
for row in input_data:
    numbers = [int(part) for part in row.split(" ")]
    matrix.append(numbers)

print(matrix)
print(solve2(matrix))
