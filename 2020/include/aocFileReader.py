import os

class aocFileReader:
    def __init__(self, dayNum: int, testInput: bool = False):
        self._dayNum = dayNum
        self._filePath = os.path.join(os.path.curdir, "day" + str(dayNum), "input", "input")
        if testInput:
            self._filePath += ".test"

    def read(self):
        lines = []
        with open(self._filePath, "r") as f:
            for l in f.readlines():
                lines.append(l.rstrip())

        f.close()
        return lines
