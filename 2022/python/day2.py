class Solution:
    def __init__(self):
        self.rules = {
            "A": {"name": "ROCK", "score": 1},
            "B": {"name": "PAPER", "score": 2},
            "C": {"name": "SCISSOR", "score": 3},
            "X": {"score": 0, "required_result": "LOSE"},
            "Y": {"score": 3, "required_result": "DRAW"},
            "Z": {"score": 6, 'required_result': "WIN"}
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

    def should_win(self, move):
        return self.rules[move]["required_result"] == "WIN"

    def should_lose(self, move):
        return self.rules[move]["required_result"] == "LOSE"

    def should_draw(self, move):
        return self.rules[move]["required_result"] == "DRAW"

    def choose_next_move(self, opponent_move, required_result):
        if self.should_win(required_result):
            if self.is_rock(opponent_move):
                return "B"
            elif self.is_paper(opponent_move):
                return "C"
            elif self.is_scissor(opponent_move):
                return "A"

        elif self.should_lose(required_result):
            if self.is_rock(opponent_move):
                return "C"

            elif self.is_paper(opponent_move):
                return "A"

            elif self.is_scissor(opponent_move):
                return "B"

        elif self.should_draw(required_result):
            if self.is_rock(opponent_move):
                return "A"
            elif self.is_paper(opponent_move):
                return "B"
            elif self.is_scissor(opponent_move):
                return "C"

    def evaluate_round(self, line):
        p1_move, required_result = line.split(" ")

        next_move = self.choose_next_move(p1_move, required_result)
        return self.rules[next_move]["score"] + self.rules[required_result]["score"]

    def solve(self):
        lines = self.get_input_lines()
        result = []
        for line in lines:
            result.append(self.evaluate_round(line.strip()))

        return sum(result)


s = Solution()
print("result of A Y move", s.evaluate_round("A Y"))
# assert(s.evaluate_round("A Y") == 4)
print(f"solution is: {s.solve()}")
