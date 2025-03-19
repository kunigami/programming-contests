import sys

a = sys.stdin.read().strip()
print(a[:10])


def expand(arr):
    expanded = []
    for i in range(len(arr)):
        if i % 2 == 0:
            label = str(i // 2)
        else:
            label = "."
        expanded += [label] * int(arr[i])
    return expanded


def compact(arr):
    left, right = 0, len(arr) - 1
    compacted = []
    while left <= right:
        if arr[right] == ".":
            right -= 1
            continue
        if arr[left] != ".":
            compacted.append(arr[left])
            left += 1
            continue

        if arr[left] == ".":
            compacted.append(arr[right])
            left += 1
            right -= 1

    return compacted


def dot_prod(arr):
    return sum([int(x) * i for i, x in enumerate(arr)])


print(len(a))
a = expand(a)
print(a)
a = compact(a)
print(a)
r = dot_prod(a)

print(r)
# 009981118882777333644655556
# 0099811188827773336446555566

# 91649267962
# 91644920634
