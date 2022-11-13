public class Commands {
    private int horizontal;
    private int depth;

    public Commands() {
        this.horizontal = 0;
        this.depth = 0;
    }

    public void addDepth(int d) { this.depth += d; }
    public void addHorizontal(int h) { this.horizontal += h; }
    public int getDepth() { return this.depth; }
    public int getHorizontal() { return this.horizontal; } 

    public int getFinalPosition() { return this.depth * this.horizontal; }
}
