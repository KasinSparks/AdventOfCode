package days.day4.part2;

import java.util.ArrayList;

import modules.AOC_FileReader;

public class Main {
    public static void main(String[] args) throws Exception {
        //AOC_FileReader reader = new AOC_FileReader("./sample_input.txt");
        AOC_FileReader reader = new AOC_FileReader("./input.txt");

        String[] lines = reader.readAllLines();

        String commaVals = lines[0];

        ArrayList<String> stringBoards = new ArrayList<String>();

        for (int i = 2; i < lines.length; ++i) {
            if (lines[i].isEmpty()) {
                continue;
            }

            stringBoards.add(lines[i]);
        }

        ArrayList<BingoBoard> boards = new ArrayList<BingoBoard>();

        for (int i = 0; i < stringBoards.size(); ++i) {
            if (i % 5 == 0) {
                // Create a new board
                boards.add(new BingoBoard());
            }

            int[] n = getNumsFromString(stringBoards.get(i));
            boards.get(boards.size() - 1).setBoardRow(i % 5, n[0], n[1], n[2], n[3], n[4]);
        }

        /*
        for (BingoBoard b : boards) {
            System.out.println(b);
        }
        */

        int[] markerVals = getNumsFromCommaString(commaVals);

        boolean hasWon = false;

        int hasWonCounter = 0;

        for (int i = 0; i < markerVals.length; ++i) {
            if (hasWon) {
                break;
            }
            //System.out.println("BINGO NUMBER: " + markerVals[i]);
            //System.out.println("+---------Boards-------+");
            for (int j = 0; j < boards.size(); ++j) {
                boards.get(j).setMark(markerVals[i]);
                //System.out.println(b);
                if (boards.get(j).checkForWin() && !boards.get(j).registerHasWon) {
                //if (boards.get(j).checkForWin()) {
                    //System.out.println("Board:");
                    //System.out.println(boards.get(j));
                    //System.out.println("HAS WON");
                    boards.get(j).registerHasWon = true;
                    if (hasWonCounter == boards.size() - 1) {
                        System.out.println("FINAL: " + boards.get(j).getFinalResult(markerVals[i]));
                    }
                    hasWonCounter++;
                }
            }

            //System.out.println("+---------END Boards-------+");
        }


    }

    public static int[] getNumsFromString(String s) {
        ArrayList<Integer> nums = new ArrayList<Integer>();

        StringBuilder sb = new StringBuilder();

        int start = 0;
        if (s.charAt(0) == ' ') {
            start = 1;
        }

        for (int i = start; i < s.length(); ++i) {

            if (s.charAt(i) == ' ') {
                if (i < s.length() - 1 && s.charAt(i + 1) == ' ') {
                    i += 1;
                }
                // new number
                nums.add(Integer.parseInt(sb.toString()));
                sb = new StringBuilder();
                continue;
            }

            sb.append(s.charAt(i));

            if (i >= s.length() - 1) {
                nums.add(Integer.parseInt(sb.toString()));
            }
        }

        int[] numArr = new int[nums.size()];
        for (int i = 0; i < numArr.length; ++i) {
            numArr[i] = nums.get(i);
        }

        return numArr;
    }

    public static int[] getNumsFromCommaString(String s) {
        ArrayList<Integer> nums = new ArrayList<Integer>();
        int lastMarker = 0;

        for (int i = 0; i < s.length(); ++i) {
            if (s.charAt(i) == ',') {
                nums.add(Integer.parseInt(s.substring(lastMarker, i)));
                lastMarker = i + 1;
            }

            if (i == s.length() - 1) {
                nums.add(Integer.parseInt(s.substring(lastMarker, i + 1)));
            }
        }

        int[] numArr = new int[nums.size()];
        for (int i = 0; i < numArr.length; ++i) {
            numArr[i] = nums.get(i);
        }

        return numArr;
    }
}
