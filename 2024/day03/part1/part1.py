from enum import Enum

#f = open("../sample_input.txt", "r")
f = open("../input.txt", "r")

class TokenType(Enum):
    CHAR = 1
    NUM  = 2
    PARENTHESIS_OPEN = 3
    PARENTHESIS_CLOSE = 4
    COMMA = 5
    OTHER = 6

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
        else:
            self.type = TokenType.OTHER

    def __str__(self):
        return "TOKEN: {}, data: {}".format(self.type, self.data)




class MullParser:
    def parse(tokens):
        mul_ins = []

        for i in range(0, len(tokens) - 6):
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
                    continue

                
                lmv = 0
                lmv_start = i + ml  + 1
                while lmv < 3:
                    if (tokens[lmv_start + lmv].type != TokenType.NUM):
                        break;
                    lmv += 1

                if lmv < 1:
                    continue


                if (tokens[lmv_start + lmv].type != TokenType.COMMA):
                    continue


                rmv = 0
                rmv_start = lmv_start + lmv + 1
                while rmv < 3:
                    if (tokens[rmv_start + rmv].type != TokenType.NUM):
                        break;
                    rmv += 1

                if rmv < 1:
                    continue

                if (tokens[rmv_start + rmv].type != TokenType.PARENTHESIS_CLOSE):
                    continue

                lmv_str = "".join(d.data for d in tokens[lmv_start:lmv_start + lmv])
                rmv_str = "".join(d.data for d in tokens[rmv_start:rmv_start + rmv])
                mul_ins.append(MulIns(int(lmv_str), int(rmv_str)))
    
        return mul_ins


            
                

class MulIns:
    def __init__(self, v1, v2):
        self.v1 = v1
        self.v2 = v2

    def mul(self):
        return self.v1 * self.v2

    def print(self):
        print("mul({},{})".format(self.v1, self.v2))


total = 0
lines = []
for l in f.readlines():
    lines.append(l.rstrip())

    tokens = []
    for c in l:
        tokens.append(Token(c))

    ins = MullParser.parse(tokens)
    
    for i in ins:
        i.print()
        total += i.mul()

print("TOTAL: {}".format(total))


