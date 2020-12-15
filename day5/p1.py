from include.aocFileReader import aocFileReader
DAY_NUM = 5

def main():
    lines = aocFileReader(DAY_NUM).read()
    testInputLines = aocFileReader(DAY_NUM, True).read()

    #print(binaryPart("FBFBBFF", 0, 127, "F", "B"))
    #print(binaryPart("RLR", 0, 7, "L", "R"))
    m = 0
    data = []
    ids = []

    for tl in lines:
        r = binaryPart(getRowData(tl), 0, 127, "F", "B")
        if r == -1:
            print(getRowData(tl))
        c = binaryPart(getColData(tl), 0, 7, "L", "R")
        data.append([r, c])
        i = getSeatID(r, c)
        ids.append(i)
        #print(r, c, i)
        if i > m:
            m = i
    
    data.sort()
    #print(data)

    print(m)
    
    ids.sort()
    print(ids)

    for i in range(ids[0], ids[-1] + 1):
        if (i not in ids) and (i - 1 in ids) and (i + 1 in ids):
            print(i)


def getRowData(line: str) -> str:
    return line[0:7]

def getColData(line: str) -> str:
    return line[7:10]

def getSeatID(row: int, col: int) -> int:
    return (row * 8) + col

def binaryPart(input: str, lowerRange: int, upperRange: int, bott: str, top: str) -> int:
    if len(input) == 0:
        if not (lowerRange == upperRange):
            print("ERROR!")
            return 
        return lowerRange

    curr = (upperRange - lowerRange) // 2
    testChar = input[0]

    if testChar == top:
        return binaryPart(input[1:], (lowerRange + curr) + 1, upperRange, bott, top)
    elif testChar == bott:
        return binaryPart(input[1:], lowerRange, (upperRange - curr) - 1, bott, top)



# Run the program
main()
