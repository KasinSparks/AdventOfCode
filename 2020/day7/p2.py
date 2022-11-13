from include.aocFileReader import aocFileReader
DAY_NUM = 7

def main(testFile: bool):
    lines = aocFileReader(DAY_NUM, testFile).read()

    bag_data = parse_bags(lines)
    #print(bag_data)
    #print(get_s_bags(bag_data, "shinygold"))
    
    #print(test(bag_data, "shinygold"))
    #print(len(test(bag_data, "shinygold")))

    print(get_bag_bags(bag_data, "shinygold"))
    print(get_all_sub_bag_count(bag_data, "shinygold"))

def get_all_sub_bag_count(bag_data, color):
    total = 0
    sub_bags = get_bag_bags(bag_data, color) 
    for s in sub_bags:
        total += int(s[0]) +  (int(s[0]) * get_all_sub_bag_count(bag_data, s[1]))

    return total

# return the sub bags of a given bag
def get_bag_bags(bag_data, test_color):
    return bag_data[test_color]


def test(bag_data, test_color):
    results = []
    for s in get_s_bags(bag_data, test_color):
        if not s in results:
            results.append(s)
        sub_result = test(bag_data, s)
        for _s in sub_result:
            if _s not in results:
                results.append(_s)

    return results

# return the bags in which bag color is a sub bag of
def get_s_bags(bag_data, color):
    result = []

    for b in bag_data:
        sub_bags = bag_data[b]
        for s in sub_bags:
            if s[1] == color and b not in result:
                result.append(b)

    return result

def parse_bags(lines):
    bag_data = {}
    for l in lines:
        sub = l.split(" ")
        bag_color = sub[0] + sub[1]

        sub_bags = []
        token_space = 4
        curr_token = 4

        while curr_token < len(sub) - 1:
            if sub[curr_token] != "no":
                sub_bags.append([sub[curr_token], sub[curr_token+1] + sub[curr_token+2]])
                curr_token += token_space
            else:
                break

        
        bag_data[bag_color] = sub_bags

    return bag_data



# Run the program
#main(True)
main(False)
