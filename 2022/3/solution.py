# Problem: https://adventofcode.com/2022/day/3

class Solution:
    def __init__(self):
        self.characters_priority = {
            'a': 1, 'b': 2, 'c': 3, 'd': 4, 'e': 5, 'f': 6, 'g': 7, 'h': 8, 'i': 9, 'j': 10,
            'k': 11, 'l': 12, 'm': 13, 'n': 14, 'o': 15, 'p': 16, 'q': 17, 'r': 18, 's': 19,
            't': 20, 'u': 21, 'v': 22, 'w': 23, 'x': 24, 'y': 25, 'z': 26, 'A': 27, 'B': 28,
            'C': 29, 'D': 30, 'E': 31, 'F': 32, 'G': 33, 'H': 34, 'I': 35, 'J': 36, 'K': 37,
            'L': 38, 'M': 39, 'N': 40, 'O': 41, 'P': 42, 'Q': 43, 'R': 44, 'S': 45, 'T': 46,
            'U': 47, 'V': 48, 'W': 49, 'X': 50, 'Y': 51, 'Z': 52
        }
        self.result = []

    def get_input_lines(self):
        data = None
        with open("input.txt") as f:
            data = f.readlines()
        return data

    def find_dupplicate_item(self, items):
        first_items_set = set()
        second_items_set = set()
        first_half, second_half = items[0:len(items)//2], items[len(items)//2:]

        for char_1, char_2 in zip(first_half, second_half):
            first_items_set.add(char_1)
            second_items_set.add(char_2)

        intersection = first_items_set.intersection(second_items_set)
        dupplicate_letter = intersection.pop()
        return dupplicate_letter

    def solve(self):
        lines = self.get_input_lines()
        for line in lines:
            dupplicate_letter = self.find_dupplicate_item(line.strip())
            priority = self.characters_priority[dupplicate_letter]
            self.result.append(priority)

        return sum(self.result)


s = Solution()
print(s.solve())
