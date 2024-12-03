from enum import Enum

#f = open("../sample_input_2.txt", "r")
f = open("../input.txt", "r")

class TokenType(Enum):
    CHAR = 1
    NUM  = 2
    PARENTHESIS_OPEN = 3
    PARENTHESIS_CLOSE = 4
    COMMA = 5
    SINGLE_QUOTE = 6
    OTHER = 7

class Token:
    def __init__(self, data):
        self.data = data
        if data.isdigit():
            self.type = TokenType.NUM
        elif data.isalpha():
            self.type = TokenType.CHAR
        elif data == '(':
            self.type = TokenType.PARENTHESIS_OPEN
        elif data == ')':
            self.type = TokenType.PARENTHESIS_CLOSE
        elif data == ',':
            self.type = TokenType.COMMA
        elif data == '\'':
            self.type = TokenType.SINGLE_QUOTE
        else:
            self.type = TokenType.OTHER

    def __str__(self):
        return "TOKEN: {}, data: {}".format(self.type, self.data)




class MullParser:
    def try_parse_do(tokens, i):

        temp = tokens[i:i+2]
        temp_str = ""
        for j in range(0, 2):
            if (temp[j].type != TokenType.CHAR):
                temp_str = ""
                break;

            temp_str += temp[j].data

        if temp_str == "do":
            ml = 2
            if (tokens[i + ml].type != TokenType.PARENTHESIS_OPEN):
                return None

            if (tokens[i + ml + 1].type != TokenType.PARENTHESIS_CLOSE):
                return None

            return DoIns()

        return None


    def try_parse_dont(tokens, i):
        temp = tokens[i:i+5]
        temp_str = ""
        for j in range(0, 3):
            if (temp[j].type != TokenType.CHAR):
                temp_str = ""
                break;

            temp_str += temp[j].data

        if (temp_str == "don"):
            if (temp[3].type != TokenType.SINGLE_QUOTE):
                temp_str = ""

            temp_str += temp[3].data
            
            if (temp[4].type != TokenType.CHAR):
                temp_str = ""

            temp_str += temp[4].data

        if temp_str == "don't":
            ml = 5
            if (tokens[i + ml].type != TokenType.PARENTHESIS_OPEN):
                return None

            if (tokens[i + ml + 1].type != TokenType.PARENTHESIS_CLOSE):
                return None

            return DontIns()

        return None

    def try_parse_mul(tokens, i):
        temp = tokens[i:i+3]
        temp_str = ""
        for j in range(0, 3):
            if (temp[j].type != TokenType.CHAR):
                temp_str = ""
                break;

            temp_str += temp[j].data
        
        if temp_str == "mul":
            ml = 3
            if (tokens[i + ml].type != TokenType.PARENTHESIS_OPEN):
                return None

            
            lmv = 0
            lmv_start = i + ml  + 1
            while lmv < 3:
                if (tokens[lmv_start + lmv].type != TokenType.NUM):
                    break;
                lmv += 1

            if lmv < 1:
                return None


            if (tokens[lmv_start + lmv].type != TokenType.COMMA):
                return None


            rmv = 0
            rmv_start = lmv_start + lmv + 1
            while rmv < 3:
                if (tokens[rmv_start + rmv].type != TokenType.NUM):
                    break;
                rmv += 1

            if rmv < 1:
                return None

            if (tokens[rmv_start + rmv].type != TokenType.PARENTHESIS_CLOSE):
                return None

            lmv_str = "".join(d.data for d in tokens[lmv_start:lmv_start + lmv])
            rmv_str = "".join(d.data for d in tokens[rmv_start:rmv_start + rmv])
            return MulIns(int(lmv_str), int(rmv_str))

        return None


    def parse(tokens):
        ins = []

        for i in range(0, len(tokens) - 6):
            mul_val = MullParser.try_parse_mul(tokens, i)
            if mul_val is not None:
                ins.append(mul_val)
                continue

            do_val = MullParser.try_parse_do(tokens, i)
            if do_val is not None:
                ins.append(do_val)
                continue

            dont_val = MullParser.try_parse_dont(tokens, i)
            if dont_val is not None:
                ins.append(dont_val)
                continue
    
        return ins


class InsType(Enum):
    MUL  = 1
    DO   = 2
    DONT = 3
            
class Ins:
    def __init__(self, itype):
        self.type = itype

class DoIns(Ins):
    def __init__(self):
        Ins.__init__(self, InsType.DO)

    def print(self):
        print("DO()")

class DontIns(Ins):
    def __init__(self):
        Ins.__init__(self, InsType.DONT)

    def print(self):
        print("DON'T()")

class MulIns(Ins):
    def __init__(self, v1, v2):
        Ins.__init__(self, InsType.MUL)
        self.v1 = v1
        self.v2 = v2

    def mul(self):
        return self.v1 * self.v2

    def print(self):
        print("MUL({},{})".format(self.v1, self.v2))


total = 0
lines = []
is_active = True

for l in f.readlines():
    lines.append(l.rstrip())

    tokens = []
    for c in l:
        tokens.append(Token(c))

    ins = MullParser.parse(tokens)
    
    for i in ins:
        i.print()
        if i.type == InsType.MUL:
            if is_active:
                total += i.mul()
        elif i.type == InsType.DO:
            is_active = True
        elif i.type == InsType.DONT:
            is_active = False 

print("TOTAL: {}".format(total))


