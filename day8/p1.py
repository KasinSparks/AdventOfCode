from include.aocFileReader import aocFileReader
DAY_NUM = 8

class op_code:
    def __init__(self, name: str, val: int = 0):
        self.name = name
        self.val = val
        self.has_been_ran = False

    def compare(self, o):
        if self.name == o.name:
            return True
        
        return False

    def set_has_been_ran(self, val: bool):
        self.has_been_ran = val

    def __str__(self):
        return self.name + ", " + str(self.val)



class op_code_reader:
    nop = op_code("nop")
    acc = op_code("acc")
    jmp = op_code("jmp")
    
    def __init__(self, program):
        self.program = program
        self.program_counter = 0
        self.acc_val = 0
        #self.loop_dect = 0

    def read(self) -> int:
        while True:
            o = self.program[self.program_counter]
            if o.has_been_ran:
                break

            if o.compare(self.nop):
                self.program_counter += 1
            elif o.compare(self.acc):
                self.acc_val += o.val
                self.program_counter += 1
            elif o.compare(self.jmp):
                self.program_counter += o.val
                #if self.loop_dect == self.program_counter:
                #    break
                #else:
                #    self.loop_dect = self.program_counter

            o.set_has_been_ran(True)

        return self.acc_val

            



def main(testFile: bool):
    lines = aocFileReader(DAY_NUM, testFile).read()

    #print(parse(lines))

    #for i in parse(lines):
    #    print(i)
   
    print(op_code_reader(parse(lines)).read())

def parse(lines: str) -> list:
    result = []
    
    for l in lines:
        sl = l.strip().split(" ")
        result.append(op_code(sl[0], int(sl[1])))

    return result
    

# Run the program
#main(True)
main(False)
