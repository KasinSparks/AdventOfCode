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
    
    d = {0 : [1]}
    for a in ad:
        d[a] = []
    
    curr = 0
    while curr < len(ad) - 3:
        yes = 0
        for i in range(1, 4):
            if ad[curr + i] - ad[curr] < 4:
                d[ad[curr]].append(ad[curr + i])
                yes += 1
            
        #curr += 1
        curr += yes
    
    print(d)
    
    lol = []
    for _d in d:
        val = d[_d]
        _l = len(val)
        


    print(depth_test(d, 1))
    inverse_dict = {v: k for v, k in d.items().__reversed__()}
    print(inverse_dict)
    print(depth_test(inverse_dict, 1))

    

    print()
    
    #for i in range(10):
        #print(dumb(i))

    count = 1
    for i in d:
        #count += depth_test(d, i)
        count *= depth_test(d, i)

    print("count: ", count)

    count = 1
    for i in d:
        #result = depth_test(d, i)
        result = depth_test(inverse_dict, i)
        if result > 1:
            print("result: ", result)
        #print("dt: ", depth_test(inverse_dict, i))

    print("count: ", count)

    print(super_dumb(d))

def super_dumb(t):
    count = 1
    for _t in t:
        l = len(t[_t])
        if l == 3:
            count *= 4
        elif l == 2:
            count *= 2

    return count

def depth_test(t, n):
    count = 1
    if len(t[n]) <= 1:
        return count
    
    for i in t[n]:
        #count += depth_test(t, i) + 1
        #print("i: ", i)
        count *= depth_test(t, i) + 1
    
    return count


def dumb(n):
    if n == 0:
        return 0
    elif n - 1 == 0:
        return 1
    elif n - 2 == 0:
        return 1
    elif n - 3 == 0:
        return 2
    return dumb(n - 1) + dumb(n - 2) + dumb(n - 3)

def build():
    return 


def check_list(ad):
    currJ = 0
    for x in ad:
        if x - currJ > 3:
            return False
        currJ = x

    return True


# Run the program
main(True)
#main(False)
