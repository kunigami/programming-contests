import sys


DELTA = [
    (-1, 0),  # N
    (0, 1),  # E
    (1, 0),  # S
    (0, -1),  # W
]


class Solver:
    def __init__(self, matrix):
        self.matrix = matrix
        self.n = len(matrix)
        self.m = len(matrix[0])
        self.pos = self.get_initial_pos()
        # populate pos
        self.dir = 0

    def next_pos(self):
        i, j = self.pos
        di, dj = DELTA[self.dir]
        return (i + di, j + dj)

    def turn(self):
        self.dir = (self.dir + 1) % 4

    def get_initial_pos(self):
        for i in range(self.n):
            for j in range(self.m):
                if self.matrix[i][j] == "^":
                    return (i, j)
        raise Exception("No initial position found")

    def is_out_of_bounds(self, pos):
        i, j = pos
        return i < 0 or i >= self.n or j < 0 or j >= self.m

    def is_crate(self, pos):
        i, j = pos
        return self.matrix[i][j] == "#"

    def mark_pos(self):
        i, j = self.pos
        self.matrix[i][j] = "X"

    def count_marks(self):
        return sum(row.count("X") for row in self.matrix)

    def solve(self):
        while not self.is_out_of_bounds(self.pos):
            self.mark_pos()
            new_pos = self.next_pos()
            if self.is_out_of_bounds(new_pos):
                break
            if self.is_crate(new_pos):
                self.turn()
            else:
                self.pos = new_pos
        return self.count_marks()


input_data = [list(row.strip()) for row in sys.stdin.readlines()]
for row in input_data:
    print(row)
print("-----")
r = Solver(input_data).solve()
print(r)
