import copy
from include.aocFileReader import aocFileReader
DAY_NUM = 18

def main(testFile: bool):
    lines = aocFileReader(DAY_NUM, testFile).read()
    
    _sum = 0
    for l in lines:
        test = Parser(l)
        val = test.expr()
        _sum += val
        print(val)

    print(_sum)
   
## Grammer
# expr        -> term term_tail
# term_tail   -> * term term_tail | eplsion
# term        -> factor factor_tail
# factor_tail -> + factor factor_tail | eplsion
# factor      -> number | ( expr )

class Parser():
    def __init__(self, line: str):
        self.stream = line.strip("\n").replace(" ", "")
        self.curr_pos = 0
        self.tokens = self.__tokenizer__()

    def __tokenizer__(self):
        tokens = []
        curr_pos = 0
        token = ""
        
        parenth_count = []

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
        return self.tokens[self.curr_pos]

    
    def expr(self):
        t = self.term()
        tt = self.term_tail()
        result = t
        print(t, tt)
        if tt is not None:
            result = t * tt

        return result

    def term(self):
        f = self.factor()
        ft = self.factor_tail()
        result = f
        if ft is not None:
            result = f + ft

        return result

    def term_tail(self):
        if self.__peak_next_token__() == "*":
            self.__consume_next_token__()
            t = self.term()
            tt = self.term_tail()
            result = t
            if tt is not None:
                result = t * tt

            return result
        else:
            return None 

    def factor(self):
        next_token = self.__peak_next_token__()
        if next_token.isnumeric():
            n = self.__consume_next_token__()
            return int(n)
        elif next_token == "(":
            self.__consume_next_token__()
            r = self.expr()
            next_token = self.__peak_next_token__()
            if next_token != ")":
                print("ERROR")
                exit()
            self.__consume_next_token__()
            return r

    def factor_tail(self):
        if self.__peak_next_token__() == "+":
            self.__consume_next_token__()
            f = self.factor()
            ft = self.factor_tail()
            result = f
            if ft is not None:
                result = f + ft
            return result 
        else:
            return None 




# Run the program
#main(True)
main(False)
