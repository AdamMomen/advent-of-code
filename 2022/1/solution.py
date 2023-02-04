from pprint import pprint


with open("input.txt", "r") as f:
    maximum_calorie = 0
    running_sum = 0
    lines = f.readlines()
    for line in lines:
        striped_line = line.strip()
        if len(striped_line) == 0:
            maximum_calorie = max(maximum_calorie, running_sum)
            print("running sum", running_sum)
            running_sum = 0
            continue
        running_sum += int(striped_line)
    print(f"maximum_calorie is {maximum_calorie}")
