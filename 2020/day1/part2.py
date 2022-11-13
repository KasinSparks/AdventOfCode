
def main():
    l = []
    with open("./input/input", "r") as f:
        l = [int(i) for i in f]

    l.sort()

    result = [] 
    # Add to 2020
    for i in l:
        for j in l:
            if 2020 - i - j in l:
                result.append(i)
                result.append(j)
                break
        if len(result) > 0:
            break
    
    print(result[0] * result[1] * (2020 - result[0] - result[1]))



main()
