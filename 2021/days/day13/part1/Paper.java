package days.day13.part1;

import modules.Grid;
import modules.Point;
import java.util.ArrayList;
import java.lang.Exception;

public class Paper {
	private Grid<Integer> _grid;
	private ArrayList<PaperFolds> _folds;

	public Paper(String[] input) {
		this._folds = new ArrayList<PaperFolds>();
		try {
			this._parseInput(input);
		} catch (Exception ex) {
			System.out.println(ex);
		}
	}

	private void _parseInput(String[] input) throws Exception {
		// For the parser
		final String searchTerm = "fold along";

		// Find the max size of the Paper
		int max_x = 0;
		int max_y = 0;

		ArrayList<Point> points = new ArrayList<Point>();

		for (String s : input) {
			if (s.length() > searchTerm.length() &&
					s.substring(0, searchTerm.length()).equals(searchTerm)) {
				// Parse the fold
				String foldData = s.substring(searchTerm.length() + 1);
				
				boolean is_horizontal = false;
				if (foldData.charAt(0) == 'x') {
					is_horizontal = true;
				}

				int pos = Integer.parseInt(foldData.substring(2));

				PaperFolds pf = new PaperFolds(is_horizontal, pos);
				
				this._folds.add(pf);
			} else {
				// Parse the point
				
				// Skip the empty line
				if (s.length() == 0) {
					continue;
				}

				// Find the comma
				int commaPos = s.length();
				for (int i = 0; i < s.length(); ++i) {
					if (s.charAt(i) == ',') {
						commaPos = i;
						break;
					}
				}

				// ERROR: did not find the comma
				if (commaPos == s.length()) {
					throw new Exception();
				}
				
				int x = Integer.parseInt(s.substring(0, commaPos));
				int y = Integer.parseInt(s.substring(commaPos + 1));

				if (max_x < x) {
					max_x = x;
				}

				if (max_y < y) {
					max_y = y;
				}

				Point p = new Point(x, y);

				points.add(p);
			}
		}
		
		System.out.println("Max x: " + max_x);
		System.out.println("Max y: " + max_y);
		
		this._grid = new Grid<Integer>(max_x + 1, max_y + 1);
		// Initalize the grid
		for (int i = 0; i < this._grid.getSize()[0]; ++i) {
			for (int j = 0; j < this._grid.getSize()[1]; ++j) {
				this._grid.set(new Point(j, i), 0);
			}
		}

		// Add the dots to the grid
		for (Point p : points) {
			this._grid.set(p, 1);
		}
	}

	public void performFolds() {
		// Set up a temperary grid to store the fold data
		Grid<Integer> newGrid;
		
		// Perform the folds
		for (PaperFolds fold : this._folds) {
			// Determine the new size of the grid

			if (fold.getIsHorizontal()) {
				newGrid = new Grid<Integer>(this._grid.getSize()[1],
						this._grid.getSize()[0] - fold.getPosition());
			} else {
				newGrid = new Grid<Integer>(this._grid.getSize()[1] - fold.getPosition(),
						this._grid.getSize()[0]);
			}
			
			
		}
	}

	@Override
	public String toString() {
		return this._grid.toString();
	}
}
