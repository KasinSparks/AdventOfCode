package days.day6.part2;

public class Lanternfish {
    private int _timer;
    private long _modifier;

    public Lanternfish() {
        this._timer = 8;
        this._modifier = 1;
    }

    public Lanternfish(int timer) {
        this._timer = timer;
        this._modifier = 1;
    }

    public Lanternfish(long modifier) {
        this._timer = 8;
        this._modifier = modifier;
    }

    public Lanternfish(int timer, long modifier) {
        this._timer = timer;
        this._modifier = modifier;
    }

    public long getModifier() { return this._modifier;}
    
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
