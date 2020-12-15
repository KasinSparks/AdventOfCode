from include.aocFileReader import aocFileReader
import copy
import itertools
DAY_NUM = 14
ADDR_LEN = 36

def main(testFile: bool):
    lines = aocFileReader(DAY_NUM, testFile).read()

    memory = {}
    curr_mask = []
    
    for l in lines:
        _input = l.split("=")
        op = _input[0].strip()
        val = _input[1].strip()
        
        if op == "mask":
            curr_mask.clear()
            for c in val:
                curr_mask.append(c)
        else:
            mem_addr = int(op.split("[")[1].split("]")[0])
            gvmr = get_value_mask_result(int(mem_addr), curr_mask)
            options = get_options(gvmr)

            assign_memory(memory, get_options(gvmr), int(val))
    
    total = 0
    for k in memory:
        total += memory[k]

    print("ANS: ", total)
    

def assign_memory(memory: dict, options: tuple, val: int) -> None:
    # Determine mem addresses
    mem_addresses = [options[0], options[0] + sum(options[1])]
    subsets = find_subsets(options[1], len(options[1]) - 1)
    for s in subsets:
        mem_addresses.append(options[0] + sum(s))

    for m in mem_addresses:
        memory[m] = val

def find_subsets(s: list, n: int) -> list:
    if n <= 0:
        return None
    subsets = list(itertools.combinations(s, n))
    sub_sub = find_subsets(s, n - 1)
    if sub_sub is not None:
        for s in sub_sub:
            subsets.append(s)

    return subsets

def get_value_mask_result(value: int, mask: list) -> list:
    b2 = pad_base2_arr_to_36bits(convert_base10_to_base2(value))
    n_val = pass_through_mask(mask, b2)
    return n_val


def get_options(r: list) -> list:
    option_list = copy.deepcopy(r)
    option_list.reverse()
    o = []
    never_change = 0
    l = len(option_list)
    for i in range(l):
        val = option_list[i]
        if val == 0:
            continue
        elif val == 1:
            never_change += 2 ** i
        elif val == "X":
            o.append(2 ** i)
        else:
            raise Exception("Unknown Val: ", option_list[i])

    return (never_change, o)



def pass_through_mask(mask: list, value: list) -> list:
    if len(mask) != len(value):
        raise Exception("mask length != value length")
    
    new_list = []
    
    for i in range(len(mask)):
        v = 0
        if mask[i] == "X":
            v = "X" 
        elif value[i] == 1 or mask[i] == '1':
            v = 1 

        new_list.insert(i, v)

    return new_list

def pad_base2_arr_to_36bits(base2_list: list) -> list:
    s = len(base2_list)
    l = copy.deepcopy(base2_list)
    for i in range(s, ADDR_LEN):
        l.insert(0, 0) 

    return l

def convert_base2_to_base10(val: list) -> int:
    n = 0
    l = len(val)
    for i in range(l):
        n += val[i] * (2 ** (l - 1 - i))

    return n

def convert_base10_to_base2(num: int) -> list:
    n = num
    l = []
    while n > 0:
        l.append(n % 2)
        n = n // 2

    l.reverse()
    return l

# Run the program
#main(True)
main(False)
