from include.aocFileReader import aocFileReader
DAY_NUM = 13

def main(testFile: bool):
    lines = aocFileReader(DAY_NUM, testFile).read()
    
    t = int(lines[0])
    raw_data = lines[1].split(',')
    
    bus_id = {}
    temp = []
    l = len(raw_data)
    for i in range(len(raw_data)):
        if raw_data[i] != 'x':
            bus_id[i] = int(raw_data[i])
            temp.append(int(raw_data[i]))
    
    print(bus_id)
    temp.sort()
    print(temp)

    off = 0
    for k,v in bus_id.items():
        if v == temp[-1]:
            off = k

    found = False
    #curr_time = temp[0]
    curr_time = temp[-1] - off
    print(curr_time)
    curr_time = 100000000000000 + (temp[0] - (100000000000000 % temp[0]))
    print(curr_time)
    ll = l - 1
    
    while not found:
        #found = is_sub(raw_data, curr_time) 
        found = is_sub2(bus_id, curr_time) 
        #curr_time += 1
        #curr_time += int(raw_data[0])
        #curr_time += (off)
        ##curr_time += temp[0] 
        curr_time += temp[-1]
        print(curr_time)
        #curr_time += ll


def is_sub(bus_list, curr_time):
    i = 0
    for i in range(len(bus_list)):
        val = bus_list[i]
        if val != 'x' and (curr_time + i) % int(val) != 0:
            return False
    
    print(curr_time)
    return True

def is_sub2(bus_list, curr_time):
    for b in bus_list:
        if (curr_time + b) % bus_list[b] != 0:
            return False
    
    print("ANWS: ", curr_time)
    return True

# Run the program
#main(True)
main(False)
