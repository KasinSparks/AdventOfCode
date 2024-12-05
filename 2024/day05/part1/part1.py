#f = open("../sample_input.txt", "r")
f = open("../input.txt", "r")


class rules():
    def parse(line):
        return line.split('|')

class updates():
    def __init__(self, rules, orderings):
        self.rules = rules
        self.orderings = orderings

    def check(self):
        seen = []
        
        for page in self.orderings:
            if page in self.rules:
                for r in self.rules[page]:
                    if r in seen:
                        return False 

            seen.append(page)

        return True


lines = []
for l in f.readlines():
    lines.append(l.rstrip())
    
parse_rules = True
orderings = []
r = {}
for l in lines:
    if l == '\n' or len(l) == 0:
        parse_rules = False
        continue

    if parse_rules:
        rule = rules.parse(l)
        if rule[0] in r:
            r[rule[0]].append(rule[1])
        else:
            r[rule[0]] = [rule[1]]
    else:
        orderings.append(l.split(','))

print(orderings)
print(r)

total = 0
for od in orderings:
    u = updates(r, od)
    print(u.check())
    if u.check():
        total += int(od[len(od) // 2])

print(total)
