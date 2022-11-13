package day4.part1;

import java.util.ArrayList;

public class BingoBoard {
    final static int BOARD_SIZE = 5;
    private int[][] _board;
    private boolean[][] _marks;

    public BingoBoard() {
        this._board = new int[BOARD_SIZE][BOARD_SIZE];
        this._marks = new boolean[BOARD_SIZE][BOARD_SIZE];

        for (int i = 0; i < this._marks.length; ++i) {
            for (int j = 0; j < this._marks.length; ++j) {
                this._marks[i][j] = false;
            }
        }
    }

    public void setBoardRow(int row, int n1, int n2, int n3, int n4, int n5) throws Exception {
        if (row > BOARD_SIZE) {
            throw new Exception("Row is out of bounds of board size.");
        }

        this._board[row] = new int[] {n1, n2, n3, n4, n5};
    }

    public int getPosition(int x, int y) throws Exception {
        this._checkBounds(x, y);

        return this._board[x][y];
    }

    public void setMark(int x, int y) throws Exception {
        this._checkBounds(x, y);

        this._marks[x][y] = true;
    }

    public void setMark(int num) throws Exception {
        for (int i = 0; i < this._board.length; ++i) {
            for (int j = 0; j < this._board[i].length; ++j) {
                if (this._board[i][j] == num) {
                    this._marks[i][j] = true;
                    break;
                }
            }
        }
    }

    public boolean checkForWin() {
        for (int i = 0; i < this._marks.length; ++i) {
            boolean b = true;
            for (int j = 0; j < this._marks.length; ++j) {
                b &= this._marks[i][j];
            }

            if (b) {
                return true;
            }
        }

        for (int i = 0; i < this._marks.length; ++i) {
            boolean b = true;
            for (int j = 0; j < this._marks.length; ++j) {
                b &= this._marks[j][i];
            }

            if (b) {
                return true;
            }
        }

        return false;
    }

    public int getFinalResult(int lastNumCalled) {
        ArrayList<Integer> nums = new ArrayList<Integer>();


        for (int i = 0; i < this._board.length; ++i) {
            for (int j = 0; j < this._board[i].length; ++j) {
                if (!this._marks[i][j]) {
                    nums.add(this._board[i][j]);
                }
            }
        }

        int sum = 0;

        for (Integer n : nums) {
            sum += n;
        }

        return sum * lastNumCalled;
    }

    private void _checkBounds(int x, int y) throws Exception {
        if (x < 0 || x > 5 || y < 0 || y > 5) {
            throw new Exception("Board position is not in range.");
        }
    }

    @Override
    public String toString() {
        StringBuilder sb = new StringBuilder();

        for (int i = 0; i < this._board.length; ++i) {
            for (int j = 0; j < this._board[i].length; ++j) {
                if (this._marks[i][j]) {
                    sb.append('X');
                } else {
                    sb.append(this._board[i][j]);
                }
                if (j < this._board[i].length - 1) {
                    sb.append(",");
                }
            }
            sb.append('\n');
        }

        return sb.toString();
    }
}