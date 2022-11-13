public class Commands {
    private int horizontal;
    private int depth;
    private int aim;

    public Commands() {
        this.horizontal = 0;
        this.depth = 0;
        this.aim = 0;
    }

    //public void addDepth(int d) { this.depth += d; }
    //public void addHorizontal(int h) { this.horizontal += h; }

    public void addAim(int a) { this.aim += a; }
    public void addForward(int n) {
        this.horizontal += n;
        this.depth += (this.aim * n);
    }

    public int getDepth() { return this.depth; }
    public int getHorizontal() { return this.horizontal; } 

    public int getFinalPosition() { return this.depth * this.horizontal; }
}
