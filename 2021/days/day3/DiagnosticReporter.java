package day3;

import java.util.ArrayList;

import modules.BinaryNumber;

public class DiagnosticReporter {
    private BinaryNumber[] _bNum;    

    public DiagnosticReporter(BinaryNumber[] bn) {
        this._bNum = bn;
    }


    public BinaryNumber getOxygenGeneratorRating() {
        ArrayList<BinaryNumber> keepers = new ArrayList<BinaryNumber>();

        for (BinaryNumber b : this._bNum) {
            keepers.add(b);
        }

        for (int col = 0; col < this._bNum[0].getBits().size(); ++col) {
            // Find most common bit in col
            int trueCount = 0;
            int falseCount = 0;
            
            for (BinaryNumber b : keepers) {
                if (b.getBits().get(col)) {
                    trueCount++;
                } else {
                    falseCount++;
                }
            }

            //System.out.println("True count: " + trueCount + ". False count: " + falseCount);

            if (falseCount > trueCount) {
                removeBinaryNumbers(keepers, false, col);
            } else {
                removeBinaryNumbers(keepers, true, col);
            }

            /*
            System.out.println("+----start----+");
            for (BinaryNumber b : keepers) {
                System.out.println(b);
            }
            System.out.println("+-----end-----+");

            System.out.println("SIZE: " + keepers.size());
            */

            if (keepers.size() == 1) {
                break;
            }
        }

        return keepers.get(0);
    }

    public BinaryNumber getC02ScrubberRating() {
        ArrayList<BinaryNumber> keepers = new ArrayList<BinaryNumber>();

        for (BinaryNumber b : this._bNum) {
            keepers.add(b);
        }

        for (int col = 0; col < this._bNum[0].getBits().size(); ++col) {
            // Find least common bit in col
            int trueCount = 0;
            int falseCount = 0;
            
            for (BinaryNumber b : keepers) {
                if (b.getBits().get(col)) {
                    trueCount++;
                } else {
                    falseCount++;
                }
            }

            if (trueCount >= falseCount) {
                removeBinaryNumbers(keepers, false, col);
            } else {
                removeBinaryNumbers(keepers, true, col);
            }

            if (keepers.size() == 1) {
                break;
            }
        }

        return keepers.get(0);
    }
    
    private void removeBinaryNumbers(ArrayList<BinaryNumber> list, boolean flag, int col) {
        // Need to go backward through the list because it will shift left when a value is removed
        for (int i = list.size() - 1; i > -1; --i) {
            if (list.get(i).getBits().get(col) != flag) {
                list.remove(i);

            }
        }
    }
}
