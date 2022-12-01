package days.day13.part1;

import modules.AOC_FileReader;

public class Main {
    public static void main(String[] args) throws Exception {
        AOC_FileReader reader = new AOC_FileReader("./sample_input.txt");
        //AOC_FileReader reader = new AOC_FileReader("./input.txt");
		
		// Read in the input
        String[] lines = reader.readAllLines();

		Paper p = new Paper(lines);

		System.out.println(p);
		
    }
}
