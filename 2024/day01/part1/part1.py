#f = open("../sample_input.txt", "r")
f = open("../input.txt", "r")
lines = []
for l in f.readlines():
    lines.append(l.rstrip())

print(lines)

left_list = []
right_list = []
for l in lines:
    ls = l.split("   ")
    left_list.append(ls[0])
    right_list.append(ls[-1])

print(left_list)
print(right_list)

left_list.sort()
right_list.sort()

print(left_list)
print(right_list)

total_dist = 0
for i in range(0, len(left_list)):
    diff = abs(int(left_list[i]) - int(right_list[i]))
    total_dist += diff

print(total_dist)
