from include.aocFileReader import aocFileReader
DAY_NUM = 9


def test_if_nth_is_valid(target_val, lines, start, n):
    nums = lines[start:start + n]

    for n in nums:
        for m in nums:
            if int(n) == int(m):
                continue
            if (int(n) + int(m)) == int(target_val):
                return True

    return False

def get_con_range(lines, target: int):
    start = 0
    end = 0
    result = 0

    while True:
        t = int(lines[start + end])
        result += int(t)

        if result < target:
            end += 1
        elif result > target:
            start += 1
            end = 0
            result = 0
        else:
            return lines[start: (start + end) + 1]






    #for i in range(len(lines)):
    #    sub_result = 0 
    #    r = []
    #    for j in range(i, len(lines)):
    #        sub_result += int(lines[j])
    #        r.append(int(lines[j]))
#
#            if len(r) > 1 and sub_result == target:
#                return r
#            elif sub_result > target:
#                sub_result = 0
#                #print(r)
#               r.clear()



def main(testFile: bool):
    lines = aocFileReader(DAY_NUM, testFile).read()

    #print(parse(lines))

    #for i in parse(lines):
    #    print(i)
    target = 0

    #n = 5
    n = 25
    for i in range(n, len(lines)):
        if not test_if_nth_is_valid(int(lines[i]), lines, i-n, n):
            target = lines[i]
            print(target)
            break

    re = get_con_range(lines, int(target))
    print(re)
    print(int(min(re)) + int(max(re)))

    #re.sort()
    #t = (re[0], re[len(re) - 1])
    #print(t)
    #print(t[0] + t[1])


def parse(lines: str) -> list:
    result = []
    
    for l in lines:
        sl = l.strip().split(" ")
        result.append(op_code(sl[0], int(sl[1])))

    return result
    

# Run the program
#main(True)
main(False)
