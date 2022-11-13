package days.day11.part1;

import java.util.ArrayList;

import modules.AOC_FileReader;
import modules.Grid;
import modules.Point;

public class Main {
    public static void main(String[] args) throws Exception {
        //AOC_FileReader reader = new AOC_FileReader("./sample_input2.txt");
        AOC_FileReader reader = new AOC_FileReader("./sample_input.txt");
        //AOC_FileReader reader = new AOC_FileReader("./input.txt");

        String[] lines = reader.readAllLines();

        ArrayList<ArrayList<Integer>> vals = new ArrayList<ArrayList<Integer>>();

        for (String l : lines) {
            vals.add(new ArrayList<>());
            for (int i = 0; i < l.length(); ++i) {
                int val = Integer.parseInt(String.valueOf(l.charAt(i)));
                vals.get(vals.size() - 1).add(val);
            }
        }

        /*
        for (int i = 0; i < vals.size(); ++i) {
            for (int j = 0; j < vals.get(i).size(); ++j) {
                System.out.print(vals.get(i).get(j));
            }
            System.out.println();
        }
        */


        Grid<Octopus> grid = new Grid<Octopus>(vals.get(0).size(), vals.size());
        for (int i = 0; i < vals.size(); ++i) {
            for (int j = 0; j < vals.get(i).size(); ++j) {
                grid.set(new Point(j, i), new Octopus(vals.get(i).get(j)));
            }
        }


        System.out.println(grid);
        System.out.println();


        for (int round = 0; round < 3; ++round) {
            // Next round
            for (int i = 0; i < vals.size(); ++i) {
                for (int j = 0; j < vals.get(i).size(); ++j) {
                    Point p = new Point(j, i);
                    grid.get(p).incrementEnergyLevel();
                }
            }
            ArrayList<Point> glowingPoints = new ArrayList<Point>();
            
            // Find the glowing points
            for (int i = 0; i < vals.size(); ++i) {
                for (int j = 0; j < vals.get(i).size(); ++j) {
                    Point p = new Point(j, i);
                    if (grid.get(p).isGlowing()) {
                        glowingPoints.add(p);
                    }
                }
            }

            System.out.println(glowingPoints.size());

            for (Point p : glowingPoints) {
                ArrayList<Point> queue = new ArrayList<Point>();
                Boolean[][] checked = new Boolean[grid.getSize()[0]][grid.getSize()[1]];
                for (int j = 0; j < checked.length; ++j) {
                    for (int k = 0; k < checked[j].length; ++k) {
                        checked[j][k] = false;
                    }
                }

                queue.add(p);

                do {
                    //System.out.println("Checking point: " + queue.get(0));
                    Point test = queue.get(0);
                    if (checked[test.getY()][test.getX()]) {
                        //System.out.println("Already checked");
                        // has already been checked
                        queue.remove(0);
                        continue;
                    }

                    checked[test.getY()][test.getX()] = true;

                    if (grid.get(test).isGlowing()) {
                        Point[] n = grid.getNeighbors(test);
                        //System.out.println("neighbors size: " + n.length);
                        for (int j = 0; j < n.length; ++j) {
                            if (checked[n[j].getY()][n[j].getX()]) {
                                // has already been checked
                                //System.out.println("Already checked");
                                continue;
                            } else if (grid.get(n[j]).isGlowing()) {
                                continue;
                            }

                            grid.get(n[j]).incrementEnergyLevel();
                            if (grid.get(n[j]).isGlowing()) {
                                checked[n[j].getY()][n[j].getX()] = false;
                            }

                            queue.add(n[j]);
                        }
                        //System.out.println("Queue size: " + queue.size());
                    } 
                    /*else {
                        grid.get(test).incrementEnergyLevel();
                    }
                    */
                    queue.remove(0);
                } while (!queue.isEmpty());
            }


            System.out.println("ROUND: " + (round + 1));
            System.out.println(grid);

            for (int i = 0; i < vals.size(); ++i) {
                for (int j = 0; j < vals.get(i).size(); ++j) {
                    Point p = new Point(j, i);
                    grid.get(p).turnOff();
                }
            }
        }


    }
}
