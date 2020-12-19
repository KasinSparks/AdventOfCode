import time
from include.aocFileReader import aocFileReader
DAY_NUM = 18

def main(testFile: bool):
    lines = aocFileReader(DAY_NUM, testFile).read()
    
    _sum = 0
    for l in lines:
        test = Parser(l)
        val = test.evaluate()
        _sum += val
        print(val)

    print(_sum)
    

class Parser():
    def __init__(self, line: str):
        self.stream = line.strip("\n").replace(" ", "")
        self.curr_pos = 0
        self.tokens = self.__tokenizer__()

    def __tokenizer__(self):
        tokens = []
        curr_pos = 0
        token = ""
        print(self.stream)
        while curr_pos < len(self.stream):
            c = self.stream[curr_pos]
            if c.isnumeric():
                token += c
            else:
                if len(token) > 0:
                    tokens.append(token)
                    token = ""
                tokens.append(c)
            curr_pos += 1
        
        # make sure we leave no tokens behind
        if len(token) > 0:
            tokens.append(token)
        
        print(tokens)
        return tokens
    
    def __consume_next_token__(self) -> str:
        if self.curr_pos >= len(self.tokens):
            return None

        t = self.tokens[self.curr_pos]
        self.curr_pos += 1
        return t

    def __peak_next_token__(self) -> str:
        if self.curr_pos >= len(self.stream):
            return None
        return line[self.curr_pos]

    def __evaluate_parenth__(self) -> int:
        result = 0
        t = self.__consume_next_token__()
        while t != ")":
            if t == "(":
                result = self.__evaluate_parenth__()
            elif t.isnumeric():
                result = int(t) 
            elif t == "+":
                result = self.__add_expr__(result)
            elif t == "*":
                result = self.__multiply_expr__(result)
            else:
                raise Exception("Unknown character: ", t)

            t = self.__consume_next_token__()

        return result

    def __add_expr__(self, curr: int):
        t = self.__consume_next_token__()
        if t == "(":
            return curr + self.__evaluate_parenth__()
        elif t.isnumeric():
            return curr + int(t)
        else:
            raise Exception("Can not add character:", t, "to", curr)

    def __multiply_expr__(self, curr: int):
        t = self.__consume_next_token__()
        if t == "(":
            return curr * self.__evaluate_parenth__()
        elif t.isnumeric():
            return curr * int(t) 
        else:
            raise Exception("Can not multiply character:", t, "to", curr)


    def evaluate(self):
        t = self.__consume_next_token__()
        result = 0
        while t is not None:
            if t == "(":
                result = self.__evaluate_parenth__()
            elif t.isnumeric():
                result = int(t)
            elif t == "+":
                result = self.__add_expr__(result)
            elif t == "*":
                result = self.__multiply_expr__(result)
            else:
                raise Exception("Unknown character: ", t)
        
            t = self.__consume_next_token__()
        return result



# Run the program
#main(True)
main(False)
