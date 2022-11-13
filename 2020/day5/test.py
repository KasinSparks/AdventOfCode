from include.aocFileReader import aocFileReader
DAY_NUM = 5

def main():
    lines = aocFileReader(DAY_NUM).read()
    testLines = aocFileReader(DAY_NUM, True).read()

    print(lines)
    print(testLines)
    

    print(lines[0])
    print(lines[0].rstrip())

    print(len(lines[0]))
    print(len(lines[0].rstrip()))



# Run the program
main()
