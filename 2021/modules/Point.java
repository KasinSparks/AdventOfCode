package modules;

public class Point {
    private int _x, _y;
    
    public Point(int x, int y) {
        this._x = x;
        this._y = y;
    }

    public Point(Point p) {
        this._x = p.getX();
        this._y = p.getY();
    }

    public int getX() { return this._x; }
    public void setX(int x) { this._x = x; }

    public int getY() { return this._y; }
    public void setY(int y) { this._y = y; }

    @Override
    public String toString() {
        return "(" + this._x + "," + this._y + ")";
    }

    @Override
    public boolean equals(Object obj) {
        if (obj == this) {
            return true;
        }
 
        if (!(obj instanceof Point)) {
            return false;
        }
         
        Point p = (Point) obj;
        return (p.getX() == this.getX()) && (p.getY() == this.getY());
    }

    public Point plus(Point p) {
        return new Point(p.getX() + this._x, p.getY() + this._y);
    }

}
