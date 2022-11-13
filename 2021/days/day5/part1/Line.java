package day5.part1;

import java.util.ArrayList;

public class Line {
    private Point _p0, _p1;

    public Line(Point p0, Point p1) {
        this._p0 = p0;
        this._p1 = p1;
    }

    public Point[] generateAllPointsOnLine() {
        ArrayList<Point> points = new ArrayList<Point>();

        // part 1 stuff
        /*
        if (this._p0.getX() != this._p1.getX() && this._p0.getY() != this._p1.getY()) {
            return null;
        }
        */

        int diffX = this._p1.getX() - this._p0.getX();
        int diffY = this._p1.getY() - this._p0.getY();

        int modifierX = 0;
        int modifierY = 0;

        if (diffX > 0) {
            modifierX = 1;
        } else if (diffX < 0) {
            modifierX = -1;
        }

        if (diffY > 0) {
            modifierY = 1;
        } else if (diffY < 0) {
            modifierY = -1;
        }

        Point walker = new Point(this._p0.getX(), this._p0.getY());
        points.add(this._p0);
        
        while (walker.getX() != this._p1.getX()
                || walker.getY() != this._p1.getY()) {
            walker.setX(walker.getX() + modifierX);
            walker.setY(walker.getY() + modifierY);
            points.add(new Point(walker.getX(), walker.getY()));
        }

        Point[] p = new Point[points.size()];

        for (int i = 0; i < p.length; ++i) {
            p[i] = points.get(i);
        }

        return p;
    }

    public Point[] getPoints() {
        return new Point[] { this._p0, this._p1 };
    }
    
    @Override
    public String toString() {
        return this._p0 + "->" + this._p1;
    }
}
