from include.aocFileReader import aocFileReader
DAY_NUM = 4

def main():
    lines = aocFileReader(DAY_NUM).read()
     
    data = parse(lines)
    count = 0
    for d in data:
        if vailidate(d):
            count += 1

    print(count)

    #print(lines)

def parse(lines: list):
    data = [{}]
    for l in lines:
        if l == "":
            data.append({})
            continue
            # End of passport
        subLine = l.split(" ")
        for sl in subLine:
            field = sl.split(":")
            data[len(data) - 1][str(field[0])] = field[1]

    return data

def vailidate(passportData: dict):
    fields = ["ecl", "pid", "eyr", "hcl", "byr", "iyr", "hgt"]
    data = []
    for f in fields:
        if not passportData.__contains__(f):
            data.append(passportData[f])
            return False
        print(data) 

    return True

# Run the program
main()
