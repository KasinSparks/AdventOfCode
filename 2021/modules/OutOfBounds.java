package modules;

public class OutOfBounds extends Exception {

    public OutOfBounds(int expectedX, int expectedY, int x, int y) {
        super("OUT OF BOUNDS. Expected: (" + expectedX + "," + expectedY
                    + ")" + ". Received: ("+ x + "," + y +")");
    }
}
