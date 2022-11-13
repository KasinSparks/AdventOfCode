package days.day9.part1;

import java.util.ArrayList;

import modules.OutOfBounds;
import modules.Point;

public class Grid {
    private int[][] _spaces;

    public Grid(int numOfCols, int numOfRows) {
        this._spaces = new int[numOfRows][numOfCols];
    }

    public int get(Point p) throws OutOfBounds {
        this._isOutOfBounds(p);
        return this._spaces[p.getY()][p.getX()];
    }
    
    public void set(Point p, int val) throws OutOfBounds {
        this._isOutOfBounds(p);
        this._spaces[p.getY()][p.getX()] = val;
    }

    public Point[] getNeigbors(Point p) throws OutOfBounds {
        this._isOutOfBounds(p);
        Point[] neigborPoints = new Point[4];
        neigborPoints[0] = new Point(0, 1);
        neigborPoints[1] = new Point(0, -1);
        neigborPoints[2] = new Point(-1, 0);
        neigborPoints[3] = new Point(1, 0);

        ArrayList<Point> vals = new ArrayList<Point>();
        for (Point n : neigborPoints) {
            try {
                this._isOutOfBounds(p.plus(n));
                vals.add(p.plus(n));
            } catch (OutOfBounds ex) {}
        }

        Point[] neigbors = new Point[vals.size()];
        for (int i = 0; i < vals.size(); ++i) {
            neigbors[i] = new Point(vals.get(i));
        }

        return neigbors;
    }

    private void _isOutOfBounds(Point p) throws OutOfBounds {
        boolean isValid = (p.getY() >= 0 && p.getY() < this._spaces.length)
                        && (p.getX() >= 0 && p.getX() < this._spaces[p.getY()].length);

        if (!isValid) {
            throw new OutOfBounds(this._spaces.length, this._spaces[0].length,
                                p.getY(), p.getX());
        }
    }
}
