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

    list_of_nums = list(ad)

    routes = get_routes(list_of_nums)
    print(routes)
    #print(traverse_routes(routes))

    # paths[n] is the total paths from 0 to n

def traverse_routes(routes):
    count = 0
    c = 0
    for i in routes:
        print("routes[i]: ", routes[i])
        l = len(routes[i])
        c += 1
        if l == 1:
            continue
        elif l > 1:
            # traverse sub routes
            res1 = dict(list(routes.items())[c:])
            count += traverse_routes(res1)
            print("res1 :", res1)
        else:
            print("here")
            count += 1
            return count 

    return count

def get_sub_routes(routes, i):
    return routes[i]

def get_routes(l):
    routes = {} 
    length = len(l)
    temp = list(l)
    temp.reverse()
    for i in range(length - 3):
        routes[temp[i]] = []
        r = routes[temp[i]]
        if temp[i] - temp[i + 1] <= 3:
            r.append(temp[i + 1])
        if temp[i] - temp[i + 2] <= 3:
            r.append(temp[i + 2])
        if temp[i] - temp[i + 3] <= 3:
            r.append(temp[i + 3])

    return routes

# Run the program
main(True)
#main(False)
