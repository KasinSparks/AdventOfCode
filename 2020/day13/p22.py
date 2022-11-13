from include.aocFileReader import aocFileReader
DAY_NUM = 13

def main(testFile: bool):
    lines = aocFileReader(DAY_NUM, testFile).read()
   
    print(solve_2(lines))
    exit()

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
    #curr_time = 100000000000000 + (temp[0] - (100000000000000 % temp[0]))
    print(curr_time)
    ll = l - 1
    curr_time = 1 
    inc = curr_time
    b_i = 0
    while b_i < len(raw_data):
        l = len(raw_data)
        for i in range(b_i, l):
            if raw_data[i] != 'x' and (curr_time + i) % int(raw_data[i]) == 0:
                inc *= int(raw_data[i])
                b_i += 1

        #curr_time += inc
        curr_time += inc

    print(curr_time)
       
#Copied from the web
def solve_2(data):
    data = [(i, int(bus_id)) for i, bus_id in enumerate(data[1].split(',')) if bus_id != 'x']
    print(data)
    jump = data[0][1]
    time_stamp = 0
    for delta, bus_id in data[1:]:
        while (time_stamp + delta) % bus_id != 0:
            time_stamp += jump
        jump *= bus_id
    return time_stamp 


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
