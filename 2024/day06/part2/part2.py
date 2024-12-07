from enum import Enum
import copy

#f = open("../sample_input.txt", "r")
f = open("../input.txt", "r")

class Cord():
    def __init__(self, x, y):
        self.x = x
        self.y = y

    def __eq__(self, other):
        return self.x == other.x and self.y == other.y

    def __add__(self, other):
        return Cord(self.x + other.x, self.y + other.y)

    def __str__(self):
        return "({}, {})".format(self.x, self.y)

class Grid():
    def __init__(self, width, height):
        self.width = width
        self.height = height
        self.data = []
        for y in range(0, self.height):
            temp = []
            for x in range(0, self.width):
                temp.append(None)
                
            self.data.append(temp)

    def _is_in_bounds(self, cord):
        if cord.x < 0 or cord.y < 0:
            return False 

        if cord.x >= self.width or cord.y >= self.height:
            return False

        return True

    def add_data(self, cord, data):
        if not self._is_in_bounds(cord):
            return
    
        self.data[cord.y][cord.x] = data

    def remove_data(self, cord):
        if not self._is_in_bounds(cord):
            return

        self.data[cord.y][cord.x] = None

    def get(self, cord):
        if not self._is_in_bounds(cord):
            return
        
        return self.data[cord.y][cord.x]


class GuardGrid(Grid):
    def __init__(self, width, height, data):
        Grid.__init__(self, width, height)
        self.curr = None

        for h in range(0, len(data)):
            for w in range(0, len(data[0])):
                if data[h][w] == '#':
                    self.add_data(Cord(w, h), Obstruction(Cord(w, h)))
                elif data[h][w] == '.':
                    continue 
                else:
                    direction = Direction.NORTH

                    if data[h][w] == '^':
                        direction = Direction.NORTH
                    elif data[h][w] == '>':
                        direction = Direction.EAST
                    elif data[h][w] == 'v':
                        direction = Direction.SOUTH
                    elif data[h][w] == '<':
                        direction = Direction.WEST
                    else:
                        print("err")

                    self.curr = GuardMovement(direction, Cord(w, h))
                    self.add_data(Cord(w, h), GuardMovement(direction, Cord(w, h)))


    def print(self):
        for y in range(0, self.height):
            temp = "" 
            for x in range(0, self.width):
                if type(self.data[y][x]) is Obstruction:
                    temp += '#'
                elif type(self.data[y][x]) is GuardMovement:
                    temp += self.data[y][x].__str__()
                elif self.data[y][x] is None:
                    temp += '.'
                else:
                    temp += '?'

            print(temp)

    def next(self):
        np = self.curr.get_next_pos()
        #print(np)
        if not self._is_in_bounds(np):
            return False

        if type(self.data[np.y][np.x]) is Obstruction:
            self.curr.r_rotate()
            return True

        self.curr = GuardMovement(self.curr.direction, self.curr.get_next_pos())
        self.add_data(self.curr.cord, self.curr)
        return True

    def count_guard_movements(self):
        count = 0
        for y in range(0, self.height):
            for x in range(0, self.width):
                if type(self.data[y][x]) is GuardMovement:
                    count += 1

        return count



class Direction():
    NORTH = Cord( 0, -1)
    SOUTH = Cord( 0, 1)
    EAST  = Cord( 1,  0)
    WEST  = Cord(-1,  0)

class GridObj():
    def __init__(self, cord):
        self.cord = cord

class Obstruction(GridObj):
    def __init__(self, cord):
        GridObj.__init__(self, cord)

class GuardMovement(GridObj):
    def __init__(self, d, cord):
        GridObj.__init__(self, cord)
        self.direction = d

    def __eq__(self, other):
        return self.direction == other.direction and self.cord == other.cord

    def __str__(self):
        if self.direction == Direction.NORTH:
            return '^'
        elif self.direction == Direction.SOUTH:
            return 'v'
        elif self.direction == Direction.EAST:
            return '>'
        elif self.direction == Direction.WEST:
            return '<'

    def get_next_pos(self):
        return self.cord + self.direction

    def r_rotate(self):
        # there is a better way of doing this
        if self.direction == Direction.NORTH:
            self.direction = Direction.EAST
        elif self.direction == Direction.EAST:
            self.direction = Direction.SOUTH
        elif self.direction == Direction.SOUTH:
            self.direction = Direction.WEST
        elif self.direction == Direction.WEST:
            self.direction = Direction.NORTH



class StartingPoint(GuardMovement):
    def __init__(self, d, cord):
        GuardMovement.__init__(self, d, cord)



lines = []
for l in f.readlines():
    lines.append(l.rstrip())


og = GuardGrid(len(lines[0]), len(lines), lines)
start_point = copy.deepcopy(og.curr.cord)
count = 0

og2 = copy.deepcopy(og)
path = []

while og2.next():
    continue

for y in range(0, len(lines)):
    for x in range(0, len(lines[0])):
        if type(og2.get(Cord(x, y))) is GuardMovement:
            path.append(Cord(x, y))

max_cycles = len(lines) * len(lines[0]) 
for p in path:
    if type(og.get(p)) is Obstruction:
        continue
    if p == start_point:
        continue

    gg = copy.deepcopy(og)
    #gg.print()
    gg.add_data(p, Obstruction(p))
    #gg.print()
    
    curr_cycles = 0
    while curr_cycles < max_cycles:
        if not gg.next():
            break

        curr_cycles += 1
    
    if curr_cycles >= max_cycles:
        count += 1

    gg.remove_data(Cord(x, y))



print("count: {}".format(count))
