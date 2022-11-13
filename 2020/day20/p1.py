from include.aocFileReader import aocFileReader
from enum import Enum
DAY_NUM = 20

class Edge(Enum):
   TOP=0,
   BOTTON=1,
   LEFT=2,
   RIGHT=3

class Direction(Enum):
    X_AXIS=0,
    Y_AXIS=1,
    RIGHT=2,
    LEFT=3

class Tile():
    def __init__(self, _id: int, data: list):
        self.id = _id
        self.data = data
        self.hashes = self.__hash_edges__()

    def __hash_edges__(self):
        edges = {}
        edges[Edge.TOP] = self.__get_tile_edge__(Edge.TOP)
        edges[Edge.BOTTON] = self.__get_tile_edge__(Edge.BOTTON)
        edges[Edge.LEFT] = self.__get_tile_edge__(Edge.LEFT)
        edges[Edge.RIGHT] = self.__get_tile_edge__(Edge.RIGHT)
        return edges

    def __get_tile_edge__(self, edge: Edge):
        if edge == Edge.TOP:
            return self.data[0]
        elif edge == Edge.BOTTON:
            return self.data[len(self.data) - 1]
        elif edge == Edge.LEFT or edge == Edge.RIGHT:
            col = 0
            if edge == Edge.RIGHT:
                col = len(self.data) - 1
            
            _edge = ""
            for d in self.data:
                _edge += d[col]
            return _edge
        
    def get_edge_hash(self, edge: Edge):
        return self.hashes[edge]

    def __reverse_hash__(self, edge: Edge) -> str:
        _hash = self.get_edge_hash(edge)
        return _hash[::-1]

    def flip(self, direction: Direction) -> None:
        if direction == Direction.X_AXIS:
            self.data = self.data[::-1]
        elif direction == Direction.Y_AXIS:
            d = []
            for _d in self.data:
                d.append(_d[::-1])
            self.data = d
        else:
            print("ERROR: Tried to filp with unknown direction", direction)

    def rotate(self, direction: Direction) -> None:
        d = []
        height = len(self.data)
        width = len(self.data[0])

        if direction == Direction.RIGHT:
            for i in range(width):
                temp = ""
                for j in range(height - 1, -1, -1):
                    temp += self.data[j][i]
                d.append(temp)
        elif direction == Direction.LEFT:
            for i in range(width - 1, -1, -1):
                temp = ""
                for j in range(height):
                    temp += self.data[j][i]
                d.append(temp)
        else:
            raise Exception("Unknown rotation direction: ", direction)

        self.data = d


    def __str__(self):
        s = ""
        for i in self.data:
            s += i + "\n"

        return s         

def main(testFile: bool):
    lines = aocFileReader(DAY_NUM, testFile).read()
    
    tiles = {}
    count = 0
    while count < len(lines):
        tile = read_in_tile(lines[count:count+11])
        tiles[tile[0]] = tile[1]
        count += 12

    print(tiles)

    hashes = {}
    
    for t in tiles:
        temp = Tile(t, tiles[t])
        print("Hashes: ", temp.hashes)
        print("Data: \n" + temp.__str__())
        temp.flip(Direction.X_AXIS)
        print("Filped X_AXIS: \n" + temp.__str__())
        temp.flip(Direction.Y_AXIS)
        print("Filped Y_AXIS: \n" + temp.__str__())
        temp.rotate(Direction.RIGHT)
        print("Rotated Right: \n" + temp.__str__())
        temp.rotate(Direction.LEFT)
        print("Rotated Left: \n" + temp.__str__())

        for h in temp.hashes:
            _h = temp.hashes[h]
            if _h not in hashes:
                hashes[_h] = []
            hashes[temp.hashes[h]].append(temp.id) 
    
    for h in hashes:
        _h = hashes[h]
        if len(_h) == 1:


    print(hashes)

def read_in_tile(lines):
    result = []
    result.append(int(lines[0].split(" ")[1].split(":")[0]))
    data = []
    for l in lines[1:]:
        temp = ""
        temp = l.replace("#", "1")
        temp = temp.replace(".", "0")
        data.append(temp)

    result.append(data)
    return result

def hash_edges(tile):
    pass

# Run the program
main(True)
#main(False)
