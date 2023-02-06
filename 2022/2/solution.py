class Solution:
    def __init__(self):
        self.rules = {
            "A": {"name": "ROCK", "score": 1},
            "B": {"name": "PAPER", "score": 2},
            "C": {"name": "SCISSOR", "score": 3},
            "X": {"name": "ROCK", "score": 1},
            "Y": {"name": "PAPER", "score": 2},
            "Z": {"name": "SCISSOR", "score": 3}
        }

    def get_input_lines(self):
        data = None
        with open("input.txt") as f:
            data = f.readlines()
        return data

    def is_rock(self, move):
        return self.rules[move]["name"] == "ROCK"

    def is_scissor(self, move):
        return self.rules[move]["name"] == "SCISSOR"

    def is_paper(self, move):
        return self.rules[move]["name"] == "PAPER"

    def evaluate_round(self, line):
        p1_move, p2_move = line.split(" ")

        # DRAW
        if self.is_rock(p1_move) and self.is_rock(p2_move) or \
                self.is_scissor(p1_move) and self.is_scissor(p2_move) or \
                self.is_paper(p1_move) and self.is_paper(p2_move):
            return 3 + self.rules[p2_move]["score"]

        # WIN
        if self.is_scissor(p1_move) and self.is_rock(p2_move) or \
                self.is_paper(p1_move) and self.is_scissor(p2_move) or\
                self.is_rock(p1_move) and self.is_paper(p2_move):
            return 6 + self.rules[p2_move]["score"]

        # Lose
        if self.is_scissor(p1_move) and self.is_paper(p2_move) or \
                self.is_rock(p1_move) and self.is_scissor(p2_move) or\
                self.is_paper(p1_move) and self.is_rock(p2_move):
            return 0 + self.rules[p2_move]["score"]

    def solve(self):
        lines = self.get_input_lines()
        result = []
        for line in lines:
            result.append(self.evaluate_round(line.strip()))

        return sum(result)


s = Solution()
print(f"solution is: {s.solve()}")
