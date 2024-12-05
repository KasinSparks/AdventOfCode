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

    def reorder(self):
        end_nodes = []
        new_ordering = []

        for page in self.orderings:
            if page not in self.rules:
                end_nodes.append(page)
            else:
                lowest_pos = len(new_ordering)
                for rule in self.rules[page]:
                    if rule in new_ordering:
                        print(rule)
                        t = new_ordering.index(rule)
                        if t < lowest_pos:
                            lowest_pos = t

                new_ordering.insert(lowest_pos, page)
        
        for node in end_nodes:
            new_ordering.append(node)

        return new_ordering



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
    if not u.check():
        new_od = u.reorder()
        u2 = updates(r, new_od)

        total += int(new_od[len(new_od) // 2])

print(total)
