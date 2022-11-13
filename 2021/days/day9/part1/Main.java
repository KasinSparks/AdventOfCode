package days.day9.part1;

import java.util.ArrayList;

import modules.AOC_FileReader;
import modules.Point;

public class Main {
    public static void main(String[] args) throws Exception {
        //AOC_FileReader reader = new AOC_FileReader("./sample_input.txt");
        AOC_FileReader reader = new AOC_FileReader("./input.txt");

        String[] lines = reader.readAllLines();

        ArrayList<ArrayList<Integer>> vals = new ArrayList<ArrayList<Integer>>();

        for (String l : lines) {
            vals.add(new ArrayList<>());
            for (int i = 0; i < l.length(); ++i) {
                int val = Integer.parseInt(String.valueOf(l.charAt(i)));
                vals.get(vals.size() - 1).add(val);
            }
        }

        for (int i = 0; i < vals.size(); ++i) {
            for (int j = 0; j < vals.get(i).size(); ++j) {
                System.out.print(vals.get(i).get(j));
            }
            System.out.println();
        }


        Grid grid = new Grid(vals.get(0).size(), vals.size());
        for (int i = 0; i < vals.size(); ++i) {
            for (int j = 0; j < vals.get(i).size(); ++j) {
                grid.set(new Point(j, i), vals.get(i).get(j));
            }
        }

        ArrayList<Point> lowPoints = new ArrayList<Point>();
        
        // Find the low points
        for (int i = 0; i < vals.size(); ++i) {
            for (int j = 0; j < vals.get(i).size(); ++j) {
                Point[] neigbors = grid.getNeigbors(new Point(j, i));
                int val = vals.get(i).get(j);
                boolean isALowPoint = true;
                for (Point n : neigbors) {
                    if (grid.get(n) <= val) {
                        isALowPoint = false;
                        break;
                    }
                }

                if (isALowPoint) {
                    lowPoints.add(new Point(j, i));
                }
            }
        }

        int sum = 0;
        for (Point p : lowPoints) {
            System.out.println(grid.get(p) + 1 + ", " + p);
            sum += grid.get(p) + 1;
        }

        for (int i = 0; i < vals.size(); ++i) {
            for (int j = 0; j < vals.get(i).size(); ++j) {
                String mod = "\033[0m";
                for (Point p : lowPoints) {
                    if (p.getY() == i && p.getX() == j) {
                        mod = "\033[;32m";
                        break;
                    }
                }
                System.out.print(mod + vals.get(i).get(j));
            }
            System.out.println();
        }


        System.out.println("Total: " + sum);
    }
}
