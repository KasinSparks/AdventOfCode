package days.day3;

import java.security.DigestException;

import javax.tools.Diagnostic;

import modules.AOC_FileReader;
import modules.BinaryNumber;

public class Part1 {
    public static void main(String[] args) {
        //AOC_FileReader reader = new AOC_FileReader("./sample_input.txt");
        AOC_FileReader reader = new AOC_FileReader("./input.txt");

        String[] lines = reader.readAllLines();

        /*
        for (String s: lines) {
            System.out.println(s);
        }
        */

        /*
        int cols = lines[0].length();
        BinaryNumber[] bn = new BinaryNumber[cols];

        for (int i = 0; i < cols; ++i) {
            bn[i] = new BinaryNumber();
        }

        for (String s: lines) {
            for (int i = 0; i < cols; ++i) {
                boolean bit = false;
                if (s.charAt(i) == '1') {
                    bit = true;
                }

                bn[i].appendBit(bit);
            }
        }

        BinaryNumber finalNum = new BinaryNumber();

        for (BinaryNumber b : bn) {
            finalNum.appendBit(b.getMostCommonBit());
            //System.out.println(b.getMostCommonBit());
        }

        System.out.println("finalNum: " + finalNum);
        long gamma = finalNum.convertToBinary();
        System.out.println(gamma);

        BinaryNumber inverse = finalNum.getInverse();

        System.out.println(inverse);
        long epsilon = inverse.convertToBinary();
        System.out.println(epsilon);

        System.out.println(gamma * epsilon);
        */

        int cols = lines[0].length();

        BinaryNumber[] bn = new BinaryNumber[lines.length];

        for (int i = 0; i < bn.length; ++i) {
            bn[i] = new BinaryNumber();
        }

        for (int i = 0; i < bn.length; ++i) {
            for (int j = 0; j < cols; ++j) {
                boolean bit = false;
                if (lines[i].charAt(j) == '1') {
                    bit = true;
                }

                bn[i].appendBit(bit);
            }
        }

        for (BinaryNumber b : bn) {
            System.out.println(b);
        }

        DiagnosticReporter dr = new DiagnosticReporter(bn);
        BinaryNumber o2Rating = dr.getOxygenGeneratorRating();
        BinaryNumber c02Rating = dr.getC02ScrubberRating();
        System.out.println("Oxygen Rating: " + o2Rating);
        System.out.println("C02 Rating: " + c02Rating);

        System.out.println("Final: " + (o2Rating.convertToBinary() * c02Rating.convertToBinary()));
    }
}
