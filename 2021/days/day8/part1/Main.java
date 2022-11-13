package days.day8.part1;

import java.util.ArrayList;

import modules.AOC_FileReader;

public class Main {
    public static void main(String[] args) throws Exception {
        //AOC_FileReader reader = new AOC_FileReader("./sample_input.txt");
        AOC_FileReader reader = new AOC_FileReader("./input.txt");

        String[] lines = reader.readAllLines();

        ArrayList<SevenSegment> inputDisplays = new ArrayList<SevenSegment>();
        ArrayList<SevenSegment> outputDisplays = new ArrayList<SevenSegment>();

        for (String l : lines) {
            boolean isOnOutput = false;
            StringBuilder sb = new StringBuilder();
            for (int i = 0; i < l.length(); ++i) {
                if (l.charAt(i) == ' ') {
                    // new display 
                    SevenSegment segment = new SevenSegment();
                    for (int j = 0; j < sb.length(); ++j) {
                        segment.turnOnSegment(sb.charAt(j));
                    }
                    if (isOnOutput) {
                        outputDisplays.add(segment);
                    } else {
                        inputDisplays.add(segment);
                    }
                    sb = new StringBuilder();
                    continue;
                } else if (l.charAt(i) == '|') {
                    isOnOutput = true;
                    continue;
                }

                sb.append(l.charAt(i));

                if (i == l.length() - 1) {
                    // at end
                    SevenSegment segment = new SevenSegment();
                    for (int j = 0; j < sb.length(); ++j) {
                        segment.turnOnSegment(sb.charAt(j));
                    }
                    if (isOnOutput) {
                        outputDisplays.add(segment);
                    } else {
                        inputDisplays.add(segment);
                    }
                }
            }
        }

        int sum = 0;
        for (SevenSegment s : outputDisplays) {
            if (s.getNumLit() == 2 || s.getNumLit() == 3 || s.getNumLit() == 4 || s.getNumLit() == 7) {
                System.out.println(s);
                sum++;
            }
        }

        System.out.println(sum);
    }
}
