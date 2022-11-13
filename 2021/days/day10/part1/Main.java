package days.day10.part1;

import modules.AOC_FileReader;

public class Main {
    public static void main(String[] args) throws Exception {
        //AOC_FileReader reader = new AOC_FileReader("./sample_input.txt");
        AOC_FileReader reader = new AOC_FileReader("./input.txt");

        String[] lines = reader.readAllLines();

        int sum = 0;
        for (String l : lines) {
            ChunkManager cm = new ChunkManager();
            Character c = cm.validate(l);
            if (c == null) { continue; }
            ChunkType ct = ChunkType.getChunkType(c);

            switch (ct) {
                case PARENTHESIS:
                    sum += 3;
                    break;
                case SQUARE_BRACKET:
                    sum += 57;
                    break;
                case CURLY_BRACE:
                    sum += 1197;
                    break;
                case ANGLE_BRACKET:
                    sum += 25137;
                    break;

                default: break;
            }
            System.out.println(sum);
        }
    }
}
