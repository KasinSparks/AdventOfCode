package days.day6.part1;

public class Lanternfish {
    private int _timer;

    public Lanternfish() {
        this._timer = 8;
    }

    public Lanternfish(int timer) {
        this._timer = timer;
    }
    
    public boolean nextDay() {
        // Reset the timer and create new fish
        if (this._timer == 0) {
            this._timer = 6;
            return true;
        }

        this._timer--;
        return false;
    }

    @Override
    public String toString() {
        return this._timer + "";
    }
}
