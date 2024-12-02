import copy

class Report:
    def __init__(self, levels):
        self.levels = levels

    def is_safe(self):
        is_dec = False
        is_inc = False

        for i in range(1, len(self.levels)):
            diff = self.levels[i - 1] - self.levels[i]
            if abs(diff) < 1 or abs(diff) > 3:
                return False
            elif diff < 0:
                is_dec = True
            elif diff > 0:
                is_inc = True
        
        if is_inc and is_dec:
            return False

        return True 

    # EWW
    def is_safe_with_dampener(self):
        is_good = self.is_safe()

        if not is_good:
            for i in range(0, len(self.levels)):
                r1 = copy.deepcopy(self)
                r1.levels.pop(i)
                if r1.is_safe():
                    return True



        return is_good


#f = open("../sample_input.txt", "r")
f = open("../input.txt", "r")

total = 0
for l in f.readlines():
    r = Report([int(level) for level in l.strip().split(" ") if level != " "])
    if r.is_safe_with_dampener():
        total += 1

print(total)
