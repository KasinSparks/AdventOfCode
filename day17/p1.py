import copy
from include.aocFileReader import aocFileReader
DAY_NUM = 17

class Cube():
    def __init__(self, x: int, y: int, z: int, val: int):
        self.x = x
        self.y = y
        self.z = z
        self.val = val

    def compare(self, cube):
        result = True

        if self.x != cube.x:
            result = False
        elif self.y != cube.y:
            result = False
        elif self.z != cube.z:
            result = False
        elif self.val != cube.val:
            result = False

        return result

    def __str__(self):
        return str(self.val)

class Block():
    # Middle cube has pos 0,0,0 (x,y,z)
    # x is left-right, y is up-down, z is depth; where +z is into, -z outwards
    #def __init__(self, width: int, height: int, depth: int, cubes: list):
    def __init__(self, cubes: list):
        self.width = [0,0]
        self.height = [0,0]
        self.depth = [0,0]

        self.cubes = dict()
        for c in cubes:
            self.add_cube(c)

    def add_cube(self, cube: Cube) -> None:
        _hash = self.__gen_hash__(cube.x, cube.y, cube.z)
        self.cubes[_hash] = cube
        
        self.__check_bounds__(cube.x, self.width)
        self.__check_bounds__(cube.y, self.height)
        self.__check_bounds__(cube.z, self.depth)

    def remove_cube(self, cube: Cube) -> None:
        _hash = self.__gen_hash__(cube.x, cube.y, cube.z)
        self.cubes.pop(_hash)

        # TODO: should maybe recalculate bounds
    

    def __check_bounds__(self, cord: int, bounds: list) -> None:
        if cord > bounds[1]:
            bounds[1] = cord 
        elif cord < bounds[0]:
            bounds[0] = cord 



    def get_cube(self, x: int, y: int, z: int) -> Cube:
        _hash = self.__gen_hash__(x, y, z)
        try:
            return self.cubes[_hash]
        except KeyError as e:
            return None

    def __gen_hash__(self, x: int, y: int, z: int) -> str:
        return str(x) + ',' + str(y) + ',' + str(z)
    
    def get_neighbors(self, x: int, y: int, z: int) -> list:
        neighbors = []

        for _x in range(-1, 2):
            for _y in range(-1, 2):
                for _z in range(-1, 2):
                    if _x == 0 and _y == 0 and _z == 0:
                        continue

                    n = self.get_cube(x + _x, y + _y, z + _z)
                    neighbors.append(n)

        return neighbors

    def print_block(self):
        count = 0
        for z in range(self.depth[0], self.depth[1] + 1):
            print("z=" + str(z))
            for y in range(self.height[0], self.height[1] + 1):
                for x in range(self.width[0], self.width[1] + 1):
                    c = self.get_cube(x, y, z)
                    if c is None:
                        print(".", end="")
                    else:
                        count += 1
                        print(c, end="")
                print()
            print("\n")

    def cycle(self):
        n_block = copy.deepcopy(self)

        for z in range(self.depth[0] - 1, self.depth[1] + 2):
            for y in range(self.height[0] - 1, self.height[1] + 2):
                for x in range(self.width[0] - 1, self.width[1] + 2):
                    c = self.get_cube(x, y, z)
                    neighbors = self.get_neighbors(x, y, z)
                    nc_active = 0
                    
                    for n in neighbors:
                        if n is not None:
                            nc_active += 1
                    
                    if c is None and nc_active == 3:
                        n_block.add_cube(Cube(x, y, z, '#'))
                    elif c is not None and (nc_active < 2 or nc_active > 3):
                        n_block.remove_cube(c)
        
        return n_block

    def get_total_num_active(self) -> int:
        return len(self.cubes)


def main(testFile: bool):
    lines = aocFileReader(DAY_NUM, testFile).read()

    block = parse_input(lines)    

    block.print_block()
    n_block = block
    for i in range(0, 6):
        n_block = n_block.cycle()
        n_block.print_block()

    print("Total: ", n_block.get_total_num_active())




def parse_input(input_lines: list) -> Block:
    block = Block([])
    y = 0
    x = 0

    for l in input_lines:
        line = l.strip()
        for _l in line:
            if _l == '#':
                block.add_cube(Cube(x, y, 0, _l))
            elif _l != '.':
                raise Exception("Unknown char", _l)

            x += 1

        x = 0
        y += 1

    return block



# Run the program
main(True)
main(False)
