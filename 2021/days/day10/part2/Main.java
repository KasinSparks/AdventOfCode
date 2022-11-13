package days.day10.part2;

import java.util.ArrayList;
import java.util.Collections;

import days.day10.part1.ChunkType;
import modules.AOC_FileReader;

public class Main {
    public static void main(String[] args) throws Exception {
        //AOC_FileReader reader = new AOC_FileReader("./sample_input.txt");
        AOC_FileReader reader = new AOC_FileReader("./input.txt");

        String[] lines = reader.readAllLines();

        ArrayList<Long> sums = new ArrayList<Long>();

        for (String l : lines) {
            long sum = 0;
            ChunkManager cm = new ChunkManager();
            ChunkType[] closers = cm.completeLine(l);

            if (closers == null) {
                continue;
            }

            for (ChunkType ct : closers) {
                System.out.print(ct + ", ");
                sum *= 5;
                switch (ct) {
                    case PARENTHESIS:
                        sum += 1;
                        break;
                    case SQUARE_BRACKET:
                        sum += 2;
                        break;
                    case CURLY_BRACE:
                        sum += 3;
                        break;
                    case ANGLE_BRACKET:
                        sum += 4;
                        break;

                    default: break;
                }
            }
            sums.add(sum);
            System.out.println(sum);
        }

        Collections.sort(sums);
        System.out.println("Final: " + sums.get(sums.size() / 2));
    }
}
