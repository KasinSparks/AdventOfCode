#f = open("../sample_input.txt", "r")
f = open("../input.txt", "r")
lines = []
for l in f.readlines():
    lines.append(l.rstrip())


left_list = []
right_list = []
for l in lines:
    ls = l.split("   ")
    left_list.append(ls[0])
    right_list.append(ls[-1])

left_list.sort()
right_list.sort()

total = 0
for ll in left_list:
    #EWW
    count = 0
    for rr in right_list:
        if ll == rr:
            count += 1

    total += int(ll) * count

print(total)
