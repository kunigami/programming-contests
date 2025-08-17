import sys


def get_words(i, j, matrix):
    n = len(matrix)
    m = len(matrix[0])
    words = []
    if j + 4 <= m:
        words.append(matrix[i][j : j + 4])

    if i + 4 <= n:
        s = ""
        for k in range(4):
            s += matrix[i + k][j]
        words.append(s)

        if j + 4 <= m:
            s = ""
            for k in range(4):
                s += matrix[i + k][j + k]
            words.append(s)
        if j - 4 >= -1:
            s = ""
            for k in range(4):
                s += matrix[i + k][j - k]
            words.append(s)
    return words


def solve(matrix):
    n = len(matrix)
    m = len(matrix[0])

    cnt = 0
    for i in range(n):
        for j in range(m):
            words = get_words(i, j, matrix)
            print(i, j, words)
            for word in words:
                if word in ("XMAS", "SAMX"):
                    cnt += 1
    return cnt


def is_ms(cc):
    return cc == "MS" or cc == "SM"


def solve2(b):
    cnt = 0
    for i in range(1, len(b) - 1):
        for j in range(1, len(b[0]) - 1):
            if b[i][j] == "A":
                if is_ms(b[i - 1][j + 1] + b[i + 1][j + 1]) and is_ms(
                    b[i - 1][j + 1] + b[i + 1][j - 1]
                ):
                    cnt += 1
    return cnt


input_data = [row.strip() for row in sys.stdin.readlines()]
print(input_data)
print(solve2(input_data))
