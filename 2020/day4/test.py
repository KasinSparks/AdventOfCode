from include.aocFileReader import aocFileReader
DAY_NUM = 4

def main():
    lines = aocFileReader(DAY_NUM).read()
    
    print(lines)

    print(lines[0])
    print(lines[0].rstrip())

    print(len(lines[0]))
    print(len(lines[0].rstrip()))



# Run the program
main()
