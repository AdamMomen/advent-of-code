from pprint import pprint


class Solution:
    def get_input_lines(self):
        data = None
        with open("input.txt") as f:
            data = f.readlines()
        return data

    def solve(self):
        result = []
        running_sum = 0
        lines = self.get_input_lines()
        for line in lines:
            striped_line = line.strip()
            if len(striped_line) == 0:
                result.append(running_sum)
                running_sum = 0
                continue
            running_sum += int(line)

        result.sort()
        print(result)
        return sum(result[len(result)-3:])


pprint(Solution().solve())
