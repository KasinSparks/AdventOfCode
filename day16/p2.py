from include.aocFileReader import aocFileReader
import copy
DAY_NUM = 16

def main(testFile: bool):
    lines = aocFileReader(DAY_NUM, testFile).read()
    pi = parse_input(lines)
    valid_tickets = get_all_valid_tickets(pi)
    valid_tickets.append(pi["my_ticket"])
    field_counter = get_field_counter_dict(pi)
    #print(pi)
    
    ticket_len = len(pi["my_ticket"])
    test = []
    for i in range(ticket_len):
        test.append(determine_field(field_counter, pi["rules"], valid_tickets, i))
    
    print(test)

    det_fields = []
    for t in test:
        #count = 0
        max_val = 0
        for w in t[1]:
            if t[1][w] > max_val:
                max_val = t[1][w]
        
        names = []
        for w in t[1]:
            #if t[1][w] > 190:
            if t[1][w] == max_val:
                #count += 1
                names.append(w)
                #print(w, ": ", t[w], end=", ")
        #print(t[0], count)
        det_fields.append((len(names), names, t[0]))

    det_fields.sort()
    print(det_fields)

    found_fields = []
    row_field = []
    for d in det_fields:
        for n in d[1]:
            if n not in found_fields:
                found_fields.append(n)
                row_field.append((d[2], n))

    print(row_field)

    rows = []
    for depart in row_field:
        if depart[1].split(" ")[0] == "departure":
            rows.append(depart[0])
    
    result = 1
    for r in rows:
        result *= pi["my_ticket"][r]

    print("RESULT: ", result)


def get_field_counter_dict(data: dict) -> dict:
    field_totals = {}
    for k in data["rules"]:
        field_totals[k] = 0
    
    return field_totals



def determine_field(field_counter: dict, rules: dict, valid_tickets: list, col_num: int) -> tuple:
    f_count = copy.deepcopy(field_counter)
    for vt in valid_tickets:
        for rule in rules:
            for _range in rules[rule]:
                f_count[rule] += 1 if check_range(vt[col_num], _range) else 0
    
    return (col_num, f_count)
    

def get_all_valid_tickets(data: dict) -> list:
    valid_tickets = []

    for ticket in data["other_tickets"]:
        is_valid = True
        for vals in ticket:
            result = check_rules(vals, data["rules"])
            if not result:
                is_valid = False
                break
        if is_valid:
            valid_tickets.append(ticket)
    
    return valid_tickets 

    
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
                parsed_data["rules"][split_val[0]] = []
                for r in ranges:
                    int_list = convert_list_to_ints(r.split("-"))
                    parsed_data["rules"][split_val[0]].append(int_list)

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
#main(True)
main(False)
