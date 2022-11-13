package day5.part1;

public class Grid {
    private int[][] _grid;
    
    public Grid(int xDim, int yDim) {
        this._grid = new int[yDim][xDim];
    }

    public int getPos(int x, int y) throws Exception {
        this._checkBounds(x, y);
        return this._grid[y][x];
    }

    public void incrementAtPos(int x, int y) throws Exception {
        this._checkBounds(x, y);
        this._grid[y][x]++;
    }

    public int numOfPointsWithOverlap() throws Exception {
        int count = 0;

        for (int i = 0; i < this._grid.length; ++i) {
            for (int j = 0; j < this._grid[i].length; ++j) {
                if (this._grid[i][j] > 1) {
                    count++;
                }
            }
        }

        return count;
    }

    @Override
    public String toString() {
        StringBuilder sb = new StringBuilder();

        for (int i = 0; i < this._grid.length; ++i) {
            for (int j = 0; j < this._grid[i].length; ++j) {
                int temp = this._grid[i][j];
                if (temp == 0) {
                    sb.append('.');
                } else {
                    sb.append(this._grid[i][j]);
                }
            }
            sb.append('\n');
        }

        return sb.toString();
    }

    private void _checkBounds(int x, int y) throws Exception {
        boolean isOutOfBouds = y < 0 || y >= this._grid.length
                    || x < 0 || x >= this._grid[y].length;

        if (isOutOfBouds) {
            throw new Exception("Parameter is out of bounds of grid ["
                                + this._grid.length + "," + this._grid[y].length
                                + "]. Got: [" + x + "," + y + "]");
        }
    }
}
