package days.day6.part2;

import java.time.Instant;
import java.util.ArrayList;

import modules.AOC_FileReader;

public class Main {
    public static void main(String[] args) throws Exception {
        System.out.println(Instant.now());
        //AOC_FileReader reader = new AOC_FileReader("./sample_input.txt");
        AOC_FileReader reader = new AOC_FileReader("./input.txt");

        String[] lines = reader.readAllLines();

        ArrayList<Integer> vals = new ArrayList<Integer>();

        for (String l : lines) {
            StringBuilder sb = new StringBuilder();
            for (int i = 0; i < l.length(); ++i) {
                if (l.charAt(i) == ',') {
                    // new num
                    vals.add(Integer.parseInt(sb.toString()));
                    sb = new StringBuilder();
                    continue;
                }

                sb.append(l.charAt(i));

                if (i == l.length() - 1) {
                    // at end
                    vals.add(Integer.parseInt(sb.toString()));
                }
            }
        }

        Lanternfish[] startingFish = new Lanternfish[vals.size()];

        for (int i = 0; i < startingFish.length; ++i) {
            startingFish[i] = new Lanternfish(vals.get(i));
        }

        FishPool pool = new FishPool(startingFish);

        // part1
        //for (int i = 0; i < 80; ++i) {
        for (int i = 0; i < 256; ++i) {
            //System.out.println("Day " + i + ": " + pool);
            pool.nextDay();
        }

        System.out.println("Fish count: " + pool.getFishCount());
        System.out.println(Instant.now());
    }
}
