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
    
    ## I WAS TAKING TOO LONG TO SOLVE THIS SO
    ## SOLUTION WAS COPIED FROM WEB
    ad.insert(0, 0)
    ad.append(ad[-1] + 3)
    print(solvePartB(ad))

tribonacciSequence = [1, 1, 2, 4, 7, 13, 24, 44, 81, 149]
def getTribonacci(num):
    if (num > len(tribonacciSequence)):
        print("Can not calc ", num, " for tribonacciSequence")
    return tribonacciSequence[num - 1]

def solvePartB(adapters):
    maxJoltage = adapters[-1]
    a = adapters

    multiplier = 1
    currentRun = 1
    for j in a:
        if (j + 1) in a:
            currentRun += 1
        else:
            multiplier *= getTribonacci(currentRun)
            currentRun = 1;

    return multiplier;


# Run the program
#main(True)
main(False)
