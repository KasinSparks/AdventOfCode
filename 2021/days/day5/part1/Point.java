package day5.part1;

public class Point {
    private int _x, _y;
    
    public Point(int x, int y) {
        this._x = x;
        this._y = y;
    }

    public int getX() { return this._x; }
    public void setX(int x) { this._x = x; }

    public int getY() { return this._y; }
    public void setY(int y) { this._y = y; }

    @Override
    public String toString() {
        return this._x + "," + this._y;
    }
}
