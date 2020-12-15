from include.aocFileReader import aocFileReader
DAY_NUM = 12

def main(testFile: bool):
    lines = aocFileReader(DAY_NUM, testFile).read()
    
    # N-S, E-W
    pos = [0,0]
    heading = 0

    for l in lines:
        _action = l[0]
        _amount = int(l[1:])
        
        if _action == "R":
            #print(turn(heading, _amount))
            heading = turn(heading, _amount)
        elif _action == "L":
            heading = turn(heading, -_amount)
        else:
            pos = travel(pos, heading, _action, _amount)

    print(pos, heading)
    print("ans: ", abs(pos[0]) + abs(pos[1]))


def travel(curr_pos, curr_heading, direction, amount):
    # This could be cleaned up
    if direction == "N":
        curr_pos[0] += amount
    elif direction == "S":
        curr_pos[0] -= amount
    elif direction == "E":
        curr_pos[1] += amount
    elif direction == "W":
        curr_pos[1] -= amount
    elif direction == "F":
        if curr_heading == 0:
            curr_pos[1] += amount 
        elif curr_heading == 1:
            curr_pos[0] -= amount 
        elif curr_heading == 2:
            curr_pos[1] -= amount 
        elif curr_heading == 3:
            curr_pos[0] += amount 
        else:
            print("Unknown direction ", direction)
    else:
        print("Unknown direction ", direction)
    
    return curr_pos

def turn(curr_heading, amount):
    turn_amount = amount / 90
    return check_heading_val(curr_heading, turn_amount)

def check_heading_val(heading, amount):
    n_val = heading + amount
    heading = n_val % 4
    return heading

# Run the program
#main(True)
main(False)
