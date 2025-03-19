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


def is_ms(c1, c2):
    if c1 == "M" and c2 == "S":
        return True
    if c1 == "S" and c2 == "M":
        return True
    return False


def solve2(matrix):
    n = len(matrix)
    m = len(matrix[0])

    cnt = 0
    for i in range(1, n - 1):
        for j in range(1, m - 1):
            if matrix[i][j] != "A":
                continue
            tl = matrix[i - 1][j - 1]
            tr = matrix[i - 1][j + 1]
            bl = matrix[i + 1][j - 1]
            br = matrix[i + 1][j + 1]
            if is_ms(tl, br) and is_ms(tr, bl):
                cnt += 1
    return cnt


input_data = [row.strip() for row in sys.stdin.readlines()]
print(input_data)
print(solve2(input_data))
