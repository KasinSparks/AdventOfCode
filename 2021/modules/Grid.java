package modules;

import java.util.ArrayList;

public class Grid<T> {
    private T[][] _spaces;

    public Grid(int numOfCols, int numOfRows) {
        this._spaces = (T[][]) new Object[numOfRows][numOfCols];
    }

    public T get(Point p) throws OutOfBounds {
        this._isOutOfBounds(p);
        return this._spaces[p.getY()][p.getX()];
    }
    
    public void set(Point p, T val) throws OutOfBounds {
        this._isOutOfBounds(p);
        this._spaces[p.getY()][p.getX()] = val;
    }

    public Point[] getAdjacentNeighbors(Point p) throws OutOfBounds {
        Point[] neighborPoints = getNeighbors(p);
        Point[] adjNeighborPoints = new Point[4];
        
        for (int i = 0; i < adjNeighborPoints.length; ++i) {
            adjNeighborPoints[i] = neighborPoints[i];
        }

        return adjNeighborPoints;
    }

    public Point[] getNeighbors(Point p) throws OutOfBounds {
        this._isOutOfBounds(p);
        Point[] neighborPoints = new Point[8];
        neighborPoints[0] = new Point(0, 1);
        neighborPoints[1] = new Point(0, -1);
        neighborPoints[2] = new Point(-1, 0);
        neighborPoints[3] = new Point(1, 0);
        neighborPoints[4] = new Point(1, 1);
        neighborPoints[5] = new Point(1, -1);
        neighborPoints[6] = new Point(-1, 1);
        neighborPoints[7] = new Point(-1, -1);

        ArrayList<Point> vals = new ArrayList<Point>();
        for (Point n : neighborPoints) {
            try {
                this._isOutOfBounds(p.plus(n));
                vals.add(p.plus(n));
            } catch (OutOfBounds ex) {}
        }

        Point[] neighbors = new Point[vals.size()];
        for (int i = 0; i < vals.size(); ++i) {
            neighbors[i] = new Point(vals.get(i));
        }

        return neighbors;
    }

    public int[] getSize() {
        int[] size = new int[2];
        size[0] = this._spaces.length;
        size[1] = this._spaces[0].length;
        return size;
    }

    private void _isOutOfBounds(Point p) throws OutOfBounds {
        boolean isValid = (p.getY() >= 0 && p.getY() < this._spaces.length)
                        && (p.getX() >= 0 && p.getX() < this._spaces[p.getY()].length);

        if (!isValid) {
            throw new OutOfBounds(this._spaces.length, this._spaces[0].length,
                                p.getY(), p.getX());
        }
    }

    @Override
    public String toString() {
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < this._spaces.length; ++i) {
            for (int j = 0; j < this._spaces[i].length; ++j) {
                sb.append(this._spaces[i][j]);
            }
            sb.append("\n");
        }

        return sb.toString();
    }
}
