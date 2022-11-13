from include.aocFileReader import aocFileReader
DAY_NUM = 6

def main(testFile: bool):
    lines = aocFileReader(DAY_NUM, testFile).read()
        
    #counts = []
    counts = []
    curr = []
    isFirst = True 
    lowest = 10000 
    lineCount = 0
    for l in lines:
        lineCount += 1

        if l == "":
            #counts.append(len(curr))
            counts.append(curr)
            curr = []
            isFirst = True
            lowest = 10000
            continue
        for sl in l:
            if lowest > len(l):
                lowest = len(l)

            if isFirst:
                curr.append(sl)
                continue
            
            removeList = []
            for c in curr:
                if lineCount >= 913 and lineCount <= 917:
                    print(c, curr)
                if not c in l:
                    #curr.remove(c)
                    removeList.append(c)

            for rl in removeList:
                curr.remove(rl)

        if len(curr) > lowest:
            print("line: " + str(lineCount) + l)
            print("ERROR: " + str(curr))


        isFirst = False

            #if not sl in curr:
            #curr.append(sl)
    
    #print(counts)
    #for c in counts:
    #    if len(c) > 0:
            #print(c)

    #for c in counts:
        

    #print(lines)
    total = 0
    for i in counts:
        total += len(i)

    print(total)




# Run the program
#main(True)
main(False)
