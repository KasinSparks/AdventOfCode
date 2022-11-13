package tests;

import modules.AOC_FileReader;

public class TEST_AOC_FileReader implements Tests {
    private AOC_FileReader _fr;

    private static final String[] FILE_CONTENTS = {
        "This is a test",
        "of the ",
        "File Readser",
        "qw3e51243tngeworuipg3h49[51j3498htgn",
        "qwrehg09pering",
        "",
        "",
        "",
        "qegn9perht",
        "",
        "",
        "qerg34t134thin]",
    };

    public TEST_AOC_FileReader() {
        this._fr = new AOC_FileReader("./assets/file_reader_test.txt");
    }

    private TestResults readLine() {
        for (int i = 0; i < FILE_CONTENTS.length; ++i) {
            if (FILE_CONTENTS[i] != this._fr.readLine()) {
                return TestResults.FAILED;
            }
        }

        return TestResults.PASSED;
    }

    private TestResults readAllLines() {
        return TestResults.FAILED;
    }

    public TestResults[] run() {
        TestResults[] tr = new TestResults[1];
        tr[0] = this.readLine();
        return tr;
    }
}
