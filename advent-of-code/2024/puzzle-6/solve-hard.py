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
        self.dir = 0
        self.marks = []

        for i in range(self.n):
            row = []
            for j in range(self.m):
                cell = [False] * 4
                row.append(cell)
            self.marks.append(row)

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
        self.marks[i][j][self.dir] = True

    def is_marked(self):
        i, j = self.pos
        return self.marks[i][j][self.dir]

    def count_marks(self):
        return sum(row.count("X") for row in self.matrix)

    def is_finite(self):
        for i in range(self.n):
            for j in range(self.m):
                for k in range(4):
                    self.marks[i][j][k] = False

        self.pos = self.get_initial_pos()
        self.dir = 0

        while not self.is_out_of_bounds(self.pos):
            if self.is_marked():
                return False
            self.mark_pos()
            new_pos = self.next_pos()
            if self.is_out_of_bounds(new_pos):
                break
            if self.is_crate(new_pos):
                self.turn()
            else:
                self.pos = new_pos
        return True

    def solve(self):
        loop_count = 0
        for i in range(self.n):
            print("row:", i)
            for j in range(self.m):
                if self.matrix[i][j] == ".":
                    self.matrix[i][j] = "#"
                    if not self.is_finite():
                        loop_count += 1
                    self.matrix[i][j] = "."
        return loop_count


input_data = [list(row.strip()) for row in sys.stdin.readlines()]
for row in input_data:
    print(row)
print("-----")
r = Solver(input_data).solve()
print(r)
