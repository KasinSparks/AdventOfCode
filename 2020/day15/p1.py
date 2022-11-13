from include.aocFileReader import aocFileReader
DAY_NUM = 15

class Num():
    def __init__(self, val, l=0, has_been_spoken=False):
        self.val = val
        self.last_spoken = l
        self.has_been_spoken = has_been_spoken

def main(testFile: bool):
    lines = aocFileReader(DAY_NUM, testFile).read()
    
    _input = []
    for l in lines:
        _input = l.split(",")

    spoken_list = {} 
    
    turn = 1
    for i in _input:
        spoken_list[int(i)] = [turn, False]
        turn += 1

    last_spoken = 0
    if last_spoken not in spoken_list:
        spoken_list[last_spoken] = [turn, False]
    spoken_list[last_spoken][1] = True

    for t in range(turn, 2020):
        if last_spoken in spoken_list:
            if spoken_list[last_spoken][1]:
                temp = spoken_list[last_spoken][0] 
                spoken_list[last_spoken][0] = t
                last_spoken = t - temp 
            else:
                temp = spoken_list[last_spoken][0] 
                spoken_list[last_spoken][0] = t
                spoken_list[last_spoken][1] = True
                last_spoken = t - temp
        else:
            spoken_list[last_spoken] = [t, True]
            last_spoken = 0
        
        #print("Turn: ", t + 1, ", last spoken: ", last_spoken)
        #print(spoken_list)
        turn = t + 1
    
    print("Turn: ", turn, ", last spoken: ", last_spoken)

    print(_input)



# Run the program
main(True)
main(False)
