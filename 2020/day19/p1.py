from include.aocFileReader import aocFileReader
DAY_NUM = 19

def main(testFile: bool):
    lines = aocFileReader(DAY_NUM, testFile).read()
    
    rules = {} 
    inputs = []

    for l in lines:
        temp = l.split(":")
        if len(l) < 1:
            continue
        elif len(temp) > 1:
            sub_rules_temp = temp[1].strip().split('|')
            sub_rules = []
            for sr in sub_rules_temp:
                t = []
                for s in sr.strip().split(" "):
                    if '\"' in s:
                        s = s.split('\"')[1].split('\"')[0]
                        sub_rules.append(s)
                        continue

                    t.append(int(s))
                sub_rules.append(tuple(t))

            subs = tuple(sub_rules)
            rules[int(temp[0])] = subs 
        else:
            inputs.append(temp)


    print(rules)
    print(inputs)
    
    rr = run_rule(0, rules)
    print(rr)

    print(combine_result(rr))



def run_rule(rule_num: int, rules: dict) -> str:
    new_str = [] 
    rule = rules[rule_num]
    if rule[0] == "a" or rule[0] == "b":
        return rule[0]
    else:
        for i in rule:
            new_new = []
            for j in i:
                print(j)
                new_new.append(run_rule(j, rules))
            print(i, new_new)
                    

            new_str.append(new_new)

    return new_str

def combine_result(result: list) -> list:
    r = ""
    for i in result:
        #print(i)
        if i == "a" or i == "b":
            r += i
        else:
            r += combine_result(i)
    
    print(r)
    return r



# Run the program
main(True)
#main(False)
