from include.aocFileReader import aocFileReader
DAY_NUM = 10


def main(testFile: bool):
    lines = aocFileReader(DAY_NUM, testFile).read()

    ad = [int(x) for x in lines]
    ad.sort()

    print(ad)

    diffs = [0,0,0]

    currJ = 0
    for x in ad:
        diffs[(x - currJ) - 1] += 1
        currJ = x
   
    # device diff
    diffs[2] += 1
    print(diffs) 

    print(diffs[0] * diffs[2])

# Run the program
#main(True)
main(False)
