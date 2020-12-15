from include.aocFileReader import aocFileReader
DAY_NUM = 6

def main(testFile: bool):
    lines = aocFileReader(DAY_NUM, testFile).read()
        
    counts = []
    curr = []
    for l in lines:
        if l == "":
            counts.append(len(curr))
            curr = []
            continue
        for sl in l:
            if not sl in curr:
                curr.append(sl)
        

    #print(lines)
    print(counts)
    total = 0
    for i in counts:
        total += i

    print(total)




# Run the program
main(True)
#main(False)
