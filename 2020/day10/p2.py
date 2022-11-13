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
    #print(diffs) 

    print(diffs[0] * diffs[2])
    
    #diffs2 = [0,0,0]
    #for i in range(len(ad) - 3):
    #    for j in range(3):
    #        if ad[i] + j + 1 == ad[i + j + 1]:
    #            diffs2[j] += 1

    #print(diffs2)


    yes = 0
    i = 0
    count = 0
    while i < (len(ad) - 3):
    #for i in range(len(ad) - 3):
        t = 0
        x = 1
        for j in range(1,4):
            if ad[i] + j == ad[i + x]:
                print(ad[i] + j, ad[i + x])
                t += 1
                x += 1

        print(t)
        
        if t == 0:
            i += 1
        else:
            i += t
            count += t 
            #count *= t 

    print(count)


# Run the program
main(True)
#main(False)
