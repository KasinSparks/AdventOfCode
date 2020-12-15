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
            return False

        data.append(passportData[f])

        val = passportData[f]
        if f == "byr":
            if not vaildYear(int(val), 1920, 2002):
                return False
        elif f == "iyr":
            if not vaildYear(int(val), 2010, 2020):
                return False
        elif f == "eyr":
            if not vaildYear(int(val), 2020, 2030):
                return False
        elif f == "hgt":
            l = len(val)
            suffex = val[l - 2] + val[l - 1]
            if not (suffex == "in" or suffex == "cm"):
                return False
            numVal = val[0:l-2]
            if suffex == "in":
                if not vaildYear(int(numVal), 59, 76):
                    return False
            else:
                if not vaildYear(int(numVal), 150, 193):
                    return False
        elif f == "hcl":
            if not len(val) == 7:
                return False
            if not val[0] == "#":
                return False
            for v in range(1,len(val)):
                if not vaildHex(val[v]):
                    return False
        elif f == "ecl":
            acceptedEcl = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
            if not val in acceptedEcl:
                return False
        elif f == "pid":
            if not len(val) == 9:
                return False
        

    return True

def vaildHex(val):
    vaildHexes = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f"]

    if val in vaildHexes:
        return True

    return False


def vaildYear(year: int, min, max):
    if year < min or year > max:
        return False

    return True


# Run the program
main()
