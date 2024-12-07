import copy

#f = open("../sample_input.txt", "r")
f = open("../input.txt", "r")


class TestValue():
    def __init__(self, vals):
        self.vals = vals

    def insert_ops(self) -> [int]:
        vals_rev = copy.deepcopy(self.vals)
        vals_rev.reverse()
        return self._insert_op(vals_rev, 0)

    def _insert_op(self, l: [int], d: int) -> [int]:
        new_vals = []
        prev_vals = []

        if d == len(l) - 1:
            return [l[d]]
 
        if d < len(l) - 1:
            prev_vals = self._insert_op(l, d + 1)
        
        for val in prev_vals:
            new_vals.append(l[d] + val)
            new_vals.append(l[d] * val)
            new_vals.append(int(str(l[d + 1]) + str(l[d])))
            new_vals.append(int(str(val) + str(l[d])))

        return new_vals

lines = []
for l in f.readlines():
    lines.append(l.rstrip())

total = 0
for l in lines:
    target = int(l.split(':')[0])
    nums = [int(x) for x in l.split(' ')[1:]]

    test_val = TestValue(nums)
    tv_io = test_val.insert_ops()
    if target in tv_io:
        total += target
    
    #print("target: {}, tv_io: {}\n".format(target, tv_io))


print("total: {}".format(total))
