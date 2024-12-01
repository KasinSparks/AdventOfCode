#f = open("../sample_input.txt", "r")
f = open("../input.txt", "r")
lines = []
for l in f.readlines():
    lines.append(l.rstrip())


left_list = {} 
right_list = {} 
for l in lines:
    ls = l.split("   ")
    if (int(ls[0]) in left_list):
        left_list[int(ls[0])] += 1
    else:
        left_list[int(ls[0])] = 1

    if (int(ls[-1]) in right_list):
        right_list[int(ls[-1])] += 1
    else:
        right_list[int(ls[-1])] = 1

total = 0
for lk in left_list:
    rv = 0
    if right_list.get(lk) != None:
        rv = right_list.get(lk)

    total += lk * left_list[lk] * rv 

print(total)
