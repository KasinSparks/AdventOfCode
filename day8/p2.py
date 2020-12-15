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

class HasBeenRanException(Exception):
    pass

class op_code_reader:
    nop = op_code("nop")
    acc = op_code("acc")
    jmp = op_code("jmp")
    
    def __init__(self, program):
        self.program = program
        self.program_counter = 0
        self.acc_val = 0

    def read(self) -> int:
        while True:
            if self.program_counter >= len(self.program):
                return self.acc_val

            o = self.program[self.program_counter]
            if o.has_been_ran:
                raise HasBeenRanException()

            if o.compare(self.nop):
                self.program_counter += 1
            elif o.compare(self.acc):
                self.acc_val += o.val
                self.program_counter += 1
            elif o.compare(self.jmp):
                self.program_counter += o.val

            o.set_has_been_ran(True)

        return self.acc_val

            
def main(testFile: bool):
    lines = aocFileReader(DAY_NUM, testFile).read()

    # change the jmp to nop one by one
    count = 0
    while True:
        program = parse(lines)
        if program[count].compare(op_code("jmp")):
            program[count] = op_code("nop", program[count].val)
        elif program[count].compare(op_code("nop")):
            program[count] = op_code("jmp", program[count].val)

        try:
            print(op_code_reader(program).read())
            break
        except HasBeenRanException:
            for i in program:
                print(i)
            count += 1
            

def parse(lines: str) -> list:
    result = []
    
    for l in lines:
        sl = l.strip().split(" ")
        result.append(op_code(sl[0], int(sl[1])))

    return result
    

# Run the program
#main(True)
main(False)
