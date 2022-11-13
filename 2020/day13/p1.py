from include.aocFileReader import aocFileReader
DAY_NUM = 13

def main(testFile: bool):
    lines = aocFileReader(DAY_NUM, testFile).read()
    
    t = int(lines[0])
    raw_data = lines[1].split(',')
    bus_ids = []
    index = 0
    for d in raw_data:
        if d != 'x':
            bus_ids.append(int(d))
        index += 1

    bus_ids.sort()
    
    m = [100000000000, bus_ids[0]]
    for b in bus_ids:
        val = b - (t % b)
        if val < m[0]:
            m[0] = val
            m[1] = b
    
    print(bus_ids)
    print(m)
    print(m[0] * m[1])


# Run the program
main(True)
main(False)
