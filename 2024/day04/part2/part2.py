#f = open("../sample_input.txt", "r")
f = open("../input.txt", "r")


def find_exs(arr):
    total = 0
    x_locs = {}

    for i in range(0, len(arr)):
        for j in range(0, len(arr[0])):
            if arr[i][j] == 'A': 
                x_locs[(i, j)] = False

    total += search_for_xmas(arr, x_locs)
    print(total)


def search_adj(arr, x, y):
    if x - 1 < 0 or x + 1 >= len(arr[0]):
        return 0

    if y - 1 < 0 or y + 1 >= len(arr):
        return 0

    a = {'M':0, 'S':0}
    for _a in a:
        if arr[y+1][x-1] != _a:
            a[_a] += 1
        if arr[y-1][x-1] != _a:
            a[_a] += 1
        if arr[y+1][x+1] != _a:
            a[_a] += 1
        if arr[y-1][x+1] != _a:
            a[_a] += 1

    if a['M'] != 2 or a['S'] != 2:
        return 0

    if arr[y+1][x+1] == 'M' and arr[y-1][x-1] == 'M':
        return 0

    if arr[y+1][x+1] == 'S' and arr[y-1][x-1] == 'S':
        return 0

    

    return 1


def search_for_xmas(arr, xlocs):
    total = 0
    for y, x in xlocs:
        print("x: {}, y: {}".format(x, y))
        total += search_adj(arr, x, y)
        xlocs[(y, x)] = True

    return total



lines = []
for l in f.readlines():
    lines.append(l.rstrip())

find_exs(lines)
