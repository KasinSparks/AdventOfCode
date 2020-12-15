from include.aocFileReader import aocFileReader
DAY_NUM = 12

def main(testFile: bool):
    lines = aocFileReader(DAY_NUM, testFile).read()
    
    pos = [0,0] # N-S, E-W
    way_point = Waypoint(1, 10)
    heading = 0

    for l in lines:
        _action = l[0]
        _amount = int(l[1:])
        
        if _action == "R" or _action == "L":
            for i in range(abs(_amount//90)):
                way_point.rot(_action)
        elif _action == "F":
            travel(pos, way_point, _amount)
        else:
            move_waypoint(way_point, _action, _amount)

        print("Pos:", pos, ", wp:", [way_point.p0, way_point.p1])

    print(pos, heading)
    print("ans: ", abs(pos[0]) + abs(pos[1]))

def move_waypoint(curr_pos, direction, amount):
    if direction == "N":
        curr_pos.p0 += amount
    elif direction == "S":
        curr_pos.p0 -= amount
    elif direction == "E":
        curr_pos.p1 += amount
    elif direction == "W":
        curr_pos.p1 -= amount
    else:
        print("Unknown direction ", direction)


def travel(curr_pos, way_point, amount):
    curr_pos[0] += way_point.p0 * amount
    curr_pos[1] += way_point.p1 * amount

class Waypoint():
    def __init__(self, p0, p1):
        self.p0 = p0
        self.p1 = p1

    def rot(self, direction):
        if direction == "R":
            self.p1 *= -1
        elif direction == "L":
            self.p0 *= -1
        else:
            print("Unknown rot direction: ", direction)

        temp = self.p0
        self.p0 = self.p1
        self.p1 = temp


# Run the program
#main(True)
main(False)
