package days.day9.part2;

import java.util.ArrayList;
import java.util.Collections;
import java.util.HashMap;

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

        // For each low point, we need to "breadth-first" search until we 
        // reach the highest point (9).
        ArrayList<Integer> basonSizes = new ArrayList<Integer>();

        // Check each low-point
        for (int i = 0; i < lowPoints.size(); ++i) {
            ArrayList<Point> queue = new ArrayList<Point>();
            Boolean[][] checked = new Boolean[grid.getSize()[0]][grid.getSize()[1]];
            System.out.println(grid.getSize()[0] + "," + grid.getSize()[1]);
            for (int j = 0; j < checked.length; ++j) {
                for (int k = 0; k < checked[j].length; ++k) {
                    checked[j][k] = false;
                }
            }

            int numChecked = 0;

            queue.add(lowPoints.get(i));

            do {
                System.out.println("Checking point: " + queue.get(0));
                Point test = queue.get(0);
                if (checked[test.getY()][test.getX()]) {
                    System.out.println("Already checked");
                    // has already been checked
                    queue.remove(0);
                    continue;
                }

                checked[test.getY()][test.getX()] = true;

                if (grid.get(queue.get(0)) < 9) {
                    numChecked++;
                    Point[] n = grid.getNeigbors(queue.get(0));
                    System.out.println("neighbors size: " + n.length);
                    for (int j = 0; j < n.length; ++j) {
                        if (checked[n[j].getY()][n[j].getX()]) {
                            // has already been checked
                            System.out.println("Already checked");
                            continue;
                        }
                        queue.add(n[j]);
                    }
                    System.out.println("Queue size: " + queue.size());
                }
                queue.remove(0);
            } while (!queue.isEmpty());

            basonSizes.add(numChecked);
        }

        for (Integer i : basonSizes) {
            System.out.println(i);
        }

        Collections.sort(basonSizes);
        int[] final_vals = new int[3];
        final_vals[0] = basonSizes.get(basonSizes.size() - 3);
        final_vals[1] = basonSizes.get(basonSizes.size() - 2);
        final_vals[2] = basonSizes.get(basonSizes.size() - 1);

        int result = 1;
        for (Integer v : final_vals) {
            result *= v;
        }

        System.out.println("Final: " + result);

        /*
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
        */
    }
}
