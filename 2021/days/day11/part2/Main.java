package days.day11.part2;

import java.util.ArrayList;
import java.util.Queue;

import modules.AOC_FileReader;
import modules.Grid;
import modules.Point;
import modules.OutOfBounds;


public class Main {
	public static void cycle(Grid<Octopus> g, ArrayList<Point> checked) {
		try {
			for (int j = 0; j < g.getSize()[0]; ++j) {
				for (int k = 0; k < g.getSize()[1]; ++k) {
					if (g.get(new Point(j, k)).isGlowing() && !checked.contains(new Point(j, k))) {
						// Update neighbors
						Point[] n = g.getNeighbors(new Point(j, k));
						for (Point p : n) {
							g.get(p).incrementEnergyLevel();
						}

						checked.add(new Point(j, k));	
					}
				}
			}
		} catch (OutOfBounds ex) {
		}
	}
    public static void main(String[] args) throws Exception {
        //AOC_FileReader reader = new AOC_FileReader("./sample_input.txt");
        AOC_FileReader reader = new AOC_FileReader("./input.txt");
		
		// Read in the input
        String[] lines = reader.readAllLines();
		
		// Determine the size of the grid and parse the string lines
        ArrayList<ArrayList<Integer>> vals = new ArrayList<ArrayList<Integer>>();

        for (String l : lines) {
            vals.add(new ArrayList<>());
            for (int i = 0; i < l.length(); ++i) {
                int val = Integer.parseInt(String.valueOf(l.charAt(i)));
                vals.get(vals.size() - 1).add(val);
            }
        }
		
		// Put the data into our octopus structure
        Grid<Octopus> grid = new Grid<Octopus>(vals.get(0).size(), vals.size());
        for (int i = 0; i < vals.size(); ++i) {
            for (int j = 0; j < vals.get(i).size(); ++j) {
                grid.set(new Point(j, i), new Octopus(vals.get(i).get(j)));
            }
        }

		// Debug print
        System.out.println(grid);
        System.out.println();

		// TODO: Run though the list multiple time finding glowing points.
		// Each time, claculate the new vaules, then add all NEW glowing points
		// to a list to be run through again and so on.
		final int NUM_OF_ROUNDS = 10000;

		System.out.println("ROUND: " + 0);
		System.out.println(grid);

		int total_flashes = 0;

		for (int i = 0; i < NUM_OF_ROUNDS; ++i) {
			ArrayList<Point> checked = new ArrayList<Point>();		
			
			for (int j = 0; j < grid.getSize()[0]; ++j) {
				for (int k = 0; k < grid.getSize()[1]; ++k) {
					grid.get(new Point(j, k)).incrementEnergyLevel();
				}
			}
			
			int num_glowing = 1;

			while (checked.size() != num_glowing) {
				System.out.println("Checked: " + checked.size() + ", Num glowing: " + num_glowing);
				cycle(grid, checked);	
				// Get number glowing
				num_glowing = 0;
				for (int j = 0; j < grid.getSize()[0]; ++j) {
					for (int k = 0; k < grid.getSize()[1]; ++k) {
						if (grid.get(new Point(j, k)).isGlowing()) {
							num_glowing++;
						}
					}
				}
			}

			if (num_glowing == grid.getSize()[0] * grid.getSize()[1]) {
				System.out.println("ROUND: " + (i + 1));
				break;
			}

			total_flashes += checked.size();

            System.out.println("ROUND: " + (i + 1));
            System.out.println(grid);

			for (int j = 0; j < grid.getSize()[0]; ++j) {
				for (int k = 0; k < grid.getSize()[1]; ++k) {
                    Point p = new Point(j, k);
					if (grid.get(p).isGlowing()) {
                    	grid.get(p).turnOff();
					}
                }
            }

		}

		System.out.println("Total: " + total_flashes);

    }
}
