import math
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
    
    #diffs2 = [0,0,0]
    #for i in range(len(ad) - 3):
    #    for j in range(3):
    #        if ad[i] + j + 1 == ad[i + j + 1]:
    #            diffs2[j] += 1

    #print(diffs2)


    #i = 0
    #count = 0
    #print("len of ad", len(ad))
    #while i < len(ad) - 1:
    #    print("ad[i + 1] - ad[i]: ", ad[i + 1] - ad[i])
    #    c = ad[i + 1] - ad[i]
    #    print("3 - c: ", 3 - c)
    #    count += 3 - c

    #    i += 1
    print(len(ad)) 
    temp = list(ad)
    i = 0
    remove_list = []
    #count = 0
    #print("len of ad", len(ad))
    j = 0
    while i < len(temp) - 2:
        print(temp[i] + 1, temp[i + 1])
        if temp[i] + 1 == temp[i + 1]:
            j += 1
            if i + 2 < len(temp) and j < 3:
                if temp[i + 2] - temp[i] < 3:
                    remove_list.append(temp[i] + 1)
            elif i + 2 >= len(temp) - 1:
                remove_list.append(temp[i] + 1)
            else:
                j = 0
        elif temp[i] + 2 == temp[i + 1]:
            j += 2
            if j < 3:
                remove_list.append(temp[i] + 1)
            else:
                j = 0
        elif temp[i] + 3 == temp[i + 1]:
            i += 1
            j = 0
            continue
        
        
        i += 1

    print(remove_list, len(remove_list))
    
    total = 0
    #for i in range(1, len(remove_list)):
    l = len(remove_list) 
    #for i in range(1, l):
        #c = math.factorial(l) / (math.factorial(i) * (math.factorial(l - i)))
    #    print(c)
    #    total += c
    #print("count: ", count)
    
    c = math.factorial(len(temp)) / (math.factorial(l) * (math.factorial(len(temp) - l)))
    print(c)
    #print(total + 2)

    #for i in range(len(remove_list)):
    #    t = list(temp)

        


    #print(check_list(temp))

def check_list(ad):
    #diffs = [0,0,0]

    currJ = 0
    for x in ad:
        if x - currJ > 3:
            return False
        #diffs[(x - currJ) - 1] += 1
        currJ = x

    return True


# Run the program
#main(True)
main(False)
