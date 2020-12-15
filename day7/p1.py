from include.aocFileReader import aocFileReader
DAY_NUM = 7



def main(testFile: bool):
    lines = aocFileReader(DAY_NUM, testFile).read()


    """
    light red bags contain 1 bright white bag, 2 muted yellow bags.
    dark orange bags contain 3 bright white bags, 4 muted yellow bags.
    bright white bags contain 1 shiny gold bag.
    muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
    shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
    dark olive bags contain 3 faded blue bags, 4 dotted black bags.
    vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
    faded blue bags contain no other bags.
    dotted black bags contain no other bags.
    """
    
    #print(bag_data)
    #print(get_matching_sub_bags(bag_data, "shinygold"))

    #for i in get_matching_sub_bags(bag_data, "shinygold"):
    #    print(get_matching_sub_bags(bag_data, i))
    #    test = get_matching_sub_bags(bag_data, i)
    #    for i in get_matching_sub_bags(bag_data, test):
    #        print(get_matching_sub_bags(bag_data, i))
    #print(search_bag(bag_data, "shinygold"))

    #find_all_sub_bags(bag_data, "shinygold")
    #print(find_all_sub_bags(bag_data, ["shinygold"]))
    bag_data = parse_bags(lines)
    #print(bag_data)
    print(get_s_bags(bag_data, "shinygold"))

    ##while not get_s_bags(bag_data, color) == None:
    
    test_colors = ["shinygold"]
    #print(test(bag_data, "shinygold"))
    print(len(test(bag_data, "shinygold")))


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

def test2(bag_data, test_color, indent=0):
    results = []
    for s in get_s_bags(bag_data, test_color):
        results.append(s)
        sub_result = test2(bag_data, s, indent+1)
        for _s in sub_result:
            if _s not in results:
                results.append(_s)
                for i in range(indent):
                    print("  ", end="")
                print(_s)

    return results



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


def find_all_sub_bags(bag_data, color):
    sub_bag_colors = [color]
    
    for c in sub_bag_colors:
        test = get_matching_sub_bags(bag_data, c)
        for t in test:
            if not test in sub_bag_colors:
                sub_bag_colors.append(test)

    print(sub_bag_colors)

    #return t
    return sub_bag_colors


def get_matching_sub_bags(bag_data, color):
    bags = []
    for k in bag_data:
        for i in bag_data[k]:
            if i[1] == color:
                bags.append(k)

    return bags


def search_bag(bag_data, color: str):
    count = 0
    bags = []
    for k in bag_data:
        for i in bag_data[k]:
            if i[1] == color:
                print(k, bag_data[k])
                bags.append(k)
                count += 1
    
    for b in bags:
        count += search_bag(bag_data, b)

    return count



# Run the program
#main(True)
main(False)
