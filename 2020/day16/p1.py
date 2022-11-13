from include.aocFileReader import aocFileReader
DAY_NUM = 16

def main(testFile: bool):
    lines = aocFileReader(DAY_NUM, testFile).read()
    pi = parse_input(lines)
    invalid_tickets = find_all_invaild_vaules(pi)

    print("Ticket scanning error rate: ", sum(invalid_tickets))
    
def find_all_invaild_vaules(data: dict) -> list:
    error_vals = []
    
    check_vals = []
    for ticket in data["other_tickets"]:
        for vals in ticket:
            check_vals.append(vals)
    
    for v in check_vals:
        result = check_rules(v, data["rules"])
        if not result:
            error_vals.append(v)

    return error_vals

def check_rules(val: int, rules: dict) -> bool:
    is_in_range = False 
    for rule in rules:
        for _range in rules[rule]:
            if check_range(val, _range):
                is_in_range = True 
    
    return is_in_range 


def check_range(value: int, _range: list[int]) -> bool:
    if _range[0] <= value and value <= _range[1]:
        return True

    return False

def parse_input(input_lines: list) -> dict:
    parsed_data = {}
    parsed_data["rules"] = {}
    parsed_data["my_ticket"] = []
    parsed_data["other_tickets"] = []

    has_parsed_rules = False
    has_parsed_my_ticket = False
    has_parsed_nearby_tickets = False

    for l in input_lines:
        line = l.strip()
        
        if len(line) == 0:
            # skip empty lines
            continue
        elif ':' in line:
            split_val = line.split(':')
            
            first_word = split_val[0].split(" ")[0]
            if first_word == "your":
                has_parsed_rules = True
                continue
            elif first_word == "nearby":
                has_parsed_my_ticket = True
                continue

            if not has_parsed_rules:
                ranges = split_val[1].split("or")
                parsed_data["rules"][first_word] = []
                for r in ranges:
                    int_list = convert_list_to_ints(r.split("-"))
                    parsed_data["rules"][first_word].append(int_list)

        elif not has_parsed_my_ticket:
            int_list = convert_list_to_ints(line.split(","))
            parsed_data["my_ticket"] = int_list 
        else:
            int_list = convert_list_to_ints(line.split(","))
            parsed_data["other_tickets"].append(int_list)


    #print("Parsed Data: ", parsed_data)
    return parsed_data

def convert_list_to_ints(l: list[str]) -> list[int]:
    new_list = []
    for _l in l:
        new_list.append(int(_l))

    return new_list


# Run the program
main(True)
main(False)
