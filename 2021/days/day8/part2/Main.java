package days.day8.part2;

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
                        segment.turnOnSegment(SegmentType.getSegmentType(sb.charAt(j)));
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
                    i++;
                    continue;
                }

                sb.append(l.charAt(i));

                if (i == l.length() - 1) {
                    // at end
                    SevenSegment segment = new SevenSegment();
                    for (int j = 0; j < sb.length(); ++j) {
                        segment.turnOnSegment(SegmentType.getSegmentType(sb.charAt(j)));
                    }
                    if (isOnOutput) {
                        outputDisplays.add(segment);
                    } else {
                        inputDisplays.add(segment);
                    }
                }
            }
        }

        /*
        int sum = 0;
        for (SevenSegment s : outputDisplays) {
            if (s.getNumLit() == 2 || s.getNumLit() == 3 || s.getNumLit() == 4 || s.getNumLit() == 7) {
                System.out.println(s);
                sum++;
            }
        }

        System.out.println(sum);
        */

        //System.out.println(inputDisplays.get(0) + ", " + outputDisplays.get(0));

        StringBuilder sb = new StringBuilder();
        final int NUM_OF_INPUT_DISPLAYS_PER_LINE = 10;
        final int NUM_OF_OUTPUT_DISPLAYS_PER_LINE = 4;
        int numOfDisplaysPerLine = NUM_OF_OUTPUT_DISPLAYS_PER_LINE + NUM_OF_INPUT_DISPLAYS_PER_LINE;
        SevenSegment[] segments = new SevenSegment[numOfDisplaysPerLine];
        int n = (inputDisplays.size() + outputDisplays.size()) / numOfDisplaysPerLine;
        int input_count = 0;
        int output_count = 0;
        int sum = 0;
        for (int j = 0; j < n; ++j) {
            for (int i = 0; i < NUM_OF_INPUT_DISPLAYS_PER_LINE; ++i) {
                segments[i] = inputDisplays.get(i + input_count);
            }
            input_count += NUM_OF_INPUT_DISPLAYS_PER_LINE;
            for (int i = 0; i < NUM_OF_OUTPUT_DISPLAYS_PER_LINE; ++i) {
                segments[i + NUM_OF_INPUT_DISPLAYS_PER_LINE] = outputDisplays.get(i + output_count);
            }
            output_count += NUM_OF_OUTPUT_DISPLAYS_PER_LINE;

            Decoder decoder = new Decoder(segments);
            System.out.print("line[" + j + "]: ");
            int[] temp = decoder.decode();
            for (Integer i : temp) {
                System.out.print(i + ", ");
            }
            System.out.println();

            int partialSum = 0;
            for (int i = temp.length - 4; i < temp.length; ++i) {
                partialSum += temp[i] * Math.pow(10, temp.length - i - 1);
            }
            sum += partialSum;
            System.out.println("partial: " + partialSum);
        }

        System.out.println("Total: " + sum);
        //System.out.println(sb);
    }
}
