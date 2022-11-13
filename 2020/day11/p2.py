import copy
from include.aocFileReader import aocFileReader
DAY_NUM = 11

# Forgot to cp file before working on p2 lol
def main(testFile: bool):
    lines = aocFileReader(DAY_NUM, testFile).read()

    seats = []
    for i in range(len(lines)):
        seats.append([])
        for _l in lines[i]:
            seats[i].append(_l)

    #print(check_los(seats, 3, 3))
    print_helper(seats)
    fill_all_seats(seats) 
    total_seat_count = count_seats(seats)
    print_helper(seats)
    
    _seats = seats
    found = False
    while not found:
        temp = simulate(_seats)
        if temp == _seats:
            #print()
            #print(_seats)
            found = True
        _seats = temp 
        #print_helper(_seats)
    
    print("seats: ", count_seats(_seats), " of ", total_seat_count)

    #print(check_los(seats, 1, 1))


    #print_helper(seats)
   # print(get_adj_seats(seats, 5, 0))
   # print(check_seat(seats, 5, 0))
    #print(get_adj_seats(seats, 0, 0))
    #print(get_adj_seats(seats, len(seats[0])-1, len(seats)-1))
    #print(get_adj_seats(seats, 0, 4))
    #print(get_adj_seats(seats, 3, 4))
#
    #print(check_seat(seats, 0, 0))
    #print(check_seat(seats, 1, 0))
    
    work_bitch = True

    #while work_bitch:
def print_helper(seats):
    for i in seats:
        print(i)

    print()

def count_seats(seats):
    count = 0
    for i in range(len(seats)):
        for j in range(len(seats[i])):
            if seats[i][j] == "#":
                count += 1

    return count


def simulate(seats):
    #sim_seats = list(seats)
    sim_seats = copy.deepcopy(seats)
    for j in range(len(seats)):
        for i in range(len(seats[j])):
            val = seats[j][i]
            if val == ".":
                continue
            #elif val == "L" and check_seat(seats, i, j) == 0:
            elif val == "L" and check_los(seats, i, j) == 0:
                sim_seats[j][i] = "#"
            elif val == "#" and check_los(seats, i, j) >= 5:
                sim_seats[j][i] = "L"

    return sim_seats



def fill_all_seats(seats):
    for i in range(len(seats)):
        for j in range(len(seats[i])):
            if seats[i][j] != ".":
                seats[i][j] = "#"

def check_seat(seats, i, j):
    adj = get_adj_seats(seats, i, j)
    s = get_seat(seats, i, j)
    is_oc = False
    if s == ".":
        return -1
    elif s == "#":
        is_oc = True
    
    adj_oc_count = 0
    for j in range(len(adj)):
        for i in adj[j]:
            #print(i)
            if i == "#":
                adj_oc_count += 1

    return adj_oc_count

def check_los(seats, i, j):
    num_of_oc = 0
    num_of_oc += check_line(seats, i, j, 1, 0)
    num_of_oc += check_line(seats, i, j, -1, 0)
    num_of_oc += check_line(seats, i, j, 0, 1)
    num_of_oc += check_line(seats, i, j, 0, -1)
    num_of_oc += check_line(seats, i, j, 1, 1) 
    num_of_oc += check_line(seats, i, j, -1, 1)
    num_of_oc += check_line(seats, i, j, 1, -1)
    num_of_oc += check_line(seats, i, j, -1, -1)

    #if i == len(seats[j]) - 1 and j == len(seats) - 1:
    #    print("i, j, num_of_oc: ", i, j, num_of_oc)
    
    return num_of_oc
    

def check_line(seats, i, j, di, dj):
    if i + di >= len(seats[j]) or i + di < 0:
        return 0 
    elif j + dj >= len(seats) or j + dj < 0:
        return 0 

    r = seats[j + dj][i + di] 
    if r == "L":
        return 0
    elif r == "#":
        return 1

    return check_line(seats, i + di, j + dj, di, dj)


def get_adj_seats(seats, i, j):
    #print("len(seats): ", len(seats))
    # Matrix subpos style
    l = i - 1
    r = i + 1
    t = j - 1
    b = j + 1

    if r > len(seats[j]):
        r = len(seats[j]) - 2
        l -= 1
    elif l < 0:
        l = 0

    if t < 0:
        t = 0
    elif b > len(seats):
        b = len(seats) - 2
        t -= 1

    sub_section = []
    _j = t
    while _j <= b:
        sub_section.append([])
        _i = l
        while _i <= r:
            #print("_j - t" , _j - t, "_j, _i", _j,_i)
            if _i == i and _j == j:
                sub_section[_j - t].append("O")
            else:
                gs = get_seat(seats, _i, _j)
                if gs != -1:
                    sub_section[_j - t].append(gs)

            _i += 1

        _j += 1

    return sub_section

def get_seat(seats, i, j):
    try:
        #return seats[j][i]
        return seats[j][i]
    except:
        return -1

# Run the program
#main(True)
main(False)
