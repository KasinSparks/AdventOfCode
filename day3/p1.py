## WARNING: THIS CODE IS NO LONGER FUNCTIONAL!
##          aocFileReader has been changed to remove the newlines from the end
##          of file lines.

from include.aocFileReader import aocFileReader
DAY_NUM = 3
xSLOPE=3
ySLOPE=1

def main():
    lines = aocFileReader(DAY_NUM).read()
    print(trav(lines, 0, 0, 0))


def trav(lines, x, y, total):
    x += xSLOPE
    y += ySLOPE 
    
    if y >= len(lines):
        return total

    if x >= len(lines[y]) - 1:
        #print("x was: " + str(x))
        x = x % (len(lines[y]) - 1)
        #print("x is now: " + str(x))
    
    #print("Trying space: " + str(x) + ", " + str(y))
    if lines[y][x] == "#":
        #print(str(x) + ", " + str(y))
        total += 1

    count = 0
    for i in range(len(lines[y]) - 1):
        if count == x:
            if lines[y][i] == "#":
                print("X", end="")
            else:
                print("O", end="")
        else:
            print(lines[y][i], end="")
        count += 1

    print("")


    return trav(lines, x, y, total)



main()
