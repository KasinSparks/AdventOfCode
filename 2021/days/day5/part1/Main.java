package days.day5.part1;

import java.util.ArrayList;


import modules.AOC_FileReader;

public class Main {
    public static void main(String[] args) throws Exception {
        //AOC_FileReader reader = new AOC_FileReader("./sample_input.txt");
        AOC_FileReader reader = new AOC_FileReader("./input.txt");

        String[] lines = reader.readAllLines();

        ArrayList<Line> lineList = new ArrayList<Line>();

        for (String l : lines) {
            Point[] p = LineParser.Parse(l);
            for (int i = 0; i < p.length; i += 2) {
                Line line = new Line(p[i], p[i + 1]);
                lineList.add(line);
            }
        }


        int xMax = 0;
        int yMax = 0; 
        for (Line l : lineList) {
            System.out.println(l);
            Point[] points = l.getPoints();
            for (Point p : points) { 
                if (p.getX() > xMax) {
                    xMax = p.getX();
                }

                if (p.getY() > yMax) {
                    yMax = p.getY();
                }
            }
        }

        Grid grid = new Grid(xMax + 1, yMax + 1);
        System.out.println("xMax: " + xMax + ", yMax: " + yMax);


        for (Line l : lineList) {
            System.out.println("Line: " + l);
            Point[] points = l.generateAllPointsOnLine();
            if (points == null) {
                continue;
            }
            System.out.println("ALL POINTS: ");
            for (Point p : points) {
                System.out.println("x: " + p.getX() + ", y: " + p.getY());
                grid.incrementAtPos(p.getX(), p.getY());
            }
        }

        System.out.println(grid);

        System.out.println("FINAL: " + grid.numOfPointsWithOverlap());
    }
}
