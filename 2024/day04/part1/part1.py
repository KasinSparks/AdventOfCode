#f = open("../sample_input.txt", "r")
f = open("../input.txt", "r")


def find_exs(arr):
    total = 0
    x_locs = {}

    for i in range(0, len(arr)):
        for j in range(0, len(arr[0])):
            if arr[i][j] == 'X': 
                x_locs[(i, j)] = False

    total += search_for_xmas(arr, x_locs)
    print(total)


def search_dir(arr, x, y, x1, y1, d):
    mx = x1 - x
    my = y1 - y
    curr = [x1, y1]

    for i in range(0, d):
        xr = curr[0] + (mx * i)
        if xr < 0 or xr >= len(arr[0]):
            return 0

        yr = curr[1] + (my * i)
        if yr < 0 or yr >= len(arr):
            return 0


    m = arr[curr[1] + (0 * my)][curr[0] + (0 * mx)] == 'M'
    a = arr[curr[1] + (1 * my)][curr[0] + (1 * mx)] == 'A'
    s = arr[curr[1] + (2 * my)][curr[0] + (2 * mx)] == 'S'
    if m and a and s:
        return 1



    return 0


def search_adj(arr, x, y):
    total = 0
    for i in range(-1, 2):
        for j in range(-1, 2):
            if i == 0 and j == 0:
                pass

            if y + j < 0 or y + j >= len(arr):
                pass

            if x + i < 0 or x + i >= len(arr[0]):
                pass


            total += search_dir(arr, x, y, x+i, y+j, 3)

    return total


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
