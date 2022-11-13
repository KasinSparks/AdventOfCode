from include.aocFileReader import aocFileReader
DAY_NUM = 13

def main(testFile: bool):
    lines = aocFileReader(DAY_NUM, testFile).read()
    
    t = int(lines[0])
    raw_data = lines[1].split(',')
    
    found = False
    curr_time = 1
    #curr_time = int(raw_data[0])
    #curr_time = 100000000000000
    while not found:
        #found = is_sub(raw_data, curr_time) 
        found = is_sub2(raw_data, curr_time) 
        #curr_time += 1
        curr_time += int(raw_data[0])
        print(curr_time)


def is_sub(bus_list, curr_time):
    i = 0
    for i in range(len(bus_list)):
        val = bus_list[i]
        if val != 'x' and (curr_time + i) % int(val) != 0:
            return False
    
    print(curr_time)
    return True

def is_sub2(bus_list, curr_time):
    l = len(bus_list)
    for i in range(l):
        val = bus_list[(l - 1) - i]
        if val != 'x' and (curr_time + (l - 1) - i) % int(val) != 0:
            return False
    
    print(curr_time)
    return True

# Run the program
main(True)
#main(False)
