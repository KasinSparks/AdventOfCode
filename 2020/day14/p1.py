from include.aocFileReader import aocFileReader
import copy
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
            assign_memory(memory, curr_mask, op, int(val))
    
    print(memory)
    total = 0
    for k in memory:
        total += memory[k]

    print(total)

def assign_memory(memory: dict, curr_mask: list, op: str, val: int) -> None:
    # Determine mem addr
    mem_addr = int(op.split("[")[1].split("]")[0])
    if not mem_addr in memory:
        memory[mem_addr] = 0

    memory[mem_addr] = get_value_mask_result(val, curr_mask)

def get_value_mask_result(value: int, mask: list) -> int:
    b2 = pad_base2_arr_to_36bits(convert_base10_to_base2(value))
    n_b2_val = pass_through_mask(mask, b2)
    n_val = convert_base2_to_base10(n_b2_val)
    return n_val


def pass_through_mask(mask: list, value: list) -> list:
    if len(mask) != len(value):
        raise Exception("mask length != value length")
    
    new_list = []

    for i in range(len(mask)):
        v = 0
        if mask[i] == "X":
            v = value[i]
        else:
            v = int(mask[i])

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
main(True)
main(False)
