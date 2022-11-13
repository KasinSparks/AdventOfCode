## WARNING: THIS CODE IS NO LONGER FUNCTIONAL!
##          aocFileReader has been changed to remove the newlines from the end
##          of file lines.
from include.aocFileReader import aocFileReader
DAY_NUM = 3

def main():
    lines = aocFileReader(DAY_NUM).read()
    #print(trav(lines, 0, 0, 0))
    result = 1
    result *= trav(lines, 0, 0, 1, 1, 0)
    result *= trav(lines, 0, 0, 3, 1, 0)
    result *= trav(lines, 0, 0, 5, 1, 0)
    result *= trav(lines, 0, 0, 7, 1, 0)
    result *= trav(lines, 0, 0, 1, 2, 0)

    print(result)
    


def trav(lines, x, y, xRate, yRate, total):
    x += xRate
    y += yRate
    
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
    #for i in range(len(lines[y]) - 1):
    #    if count == x:
    #        if lines[y][i] == "#":
    #            print("X", end="")
    #        else:
    #            print("O", end="")
    #    else:
    #        print(lines[y][i], end="")
    #    count += 1

    #print("")


    return trav(lines, x, y, xRate, yRate, total)



main()
