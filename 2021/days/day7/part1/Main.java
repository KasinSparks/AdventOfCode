package days.day7.part1;

import java.util.ArrayList;

import modules.AOC_FileReader;

public class Main {
    public static void main(String[] args) throws Exception {
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

        Aligner aligner = new Aligner();

        for (Integer n : vals) {
            aligner.addSub(new CrabSubmarine(n));
        }

        System.out.println(aligner.align());
    }
}
