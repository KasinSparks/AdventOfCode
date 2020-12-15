
def main():
    with open("./input/input", "r") as f:
        l = [int(i) for i in f]
        l.sort()

        result = -1
        # Add to 2020
        for i in l:
            if 2020 - i in l:
                result = i
                break

        if (result == -1):
            print("No Match...")
            exit()

        print(result * (2020 - result))


                

        #print(l)
        #for l in f:
            #print(int(l))
            

main()
