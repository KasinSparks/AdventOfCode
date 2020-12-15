from include.aocFileReader import aocFileReader
DAY_NUM = 7

class bag():
    def __init__(self, color: str, sub_bags: list, count: int = 0):
        self.color = color
        self.sub_bags = sub_bags
        self.count = count



    

def main(test_file: bool):
    lines = aocFileReader(DAY_NUM, test_file).read()
    print(lines)



main(True)
#main(False)
