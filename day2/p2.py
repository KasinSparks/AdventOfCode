def main():
    with open("./input/input", "r") as f:
        lines = f.readlines()
        count = 0
        for l in lines:
            if checkPwd(parseLine(l)):
                count += 1

    print(count)


def parseLine(line: str):
    rawData = line.split(" ")
    occurs = rawData[0].split("-")
    parsedData = {
        "min": int(occurs[0]),
        "max": int(occurs[1]),
        "letter": rawData[1].split(":")[0],
        "pwd": rawData[2]
    }
    return parsedData


def checkPwd(parsedData: dict):
    password = parsedData["pwd"]
    f = parsedData["min"] - 1
    l = parsedData["max"] - 1
    lett = parsedData["letter"]
    if ((password[f] == lett) ^ (password[l] == lett)):
        return True

    return False 


main()
