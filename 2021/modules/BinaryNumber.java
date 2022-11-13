package modules;
import java.util.ArrayList;

public class BinaryNumber {
    private ArrayList<Boolean> _bits;

    public BinaryNumber() {
        this._bits = new ArrayList<Boolean>();
    }

    public void appendBit(boolean bit) {
        this._bits.add(bit);
    }

    public ArrayList<Boolean> getBits() {
        return this._bits;
    }

    public boolean getMostCommonBit() {
        int trueCount = 0;
        int falseCount = 0;
        for (Boolean b : this._bits) {
            if (b) {
                trueCount++;
            } else {
                falseCount++;
            }
        }

        if (trueCount > falseCount) {
            return true;
        }
        
        // TODO: edge case, counts are equal
        return false;
    }

    // TODO: Change name to convert binary to decimal
    public long convertToBinary() {
        long sum = 0;
        for (int i = 0; i < this._bits.size(); ++i) {
            sum += Math.pow(2, (this._bits.size() - i - 1)) * (this._bits.get(i) ? 1 : 0);
        }

        return sum;
    }

    public BinaryNumber getInverse() {
        BinaryNumber bn = new BinaryNumber();

        for (int i = 0; i < this._bits.size(); ++i) {
            bn.appendBit(!this._bits.get(i));
        }

        return bn;
    }

    @Override
    public String toString() {
        StringBuilder sb = new StringBuilder();
        for (Boolean b : this._bits) {
            sb.append(b ? '1' : '0');
        }

        return sb.toString();
    }
}
