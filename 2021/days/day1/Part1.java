import java.util.ArrayList;
import java.util.Scanner;
import java.io.BufferedReader;
import java.io.FileNotFoundException;
import java.io.FileReader;
import java.io.IOException;

public class Main {
    public static void main(String[] args) {
        FileReader reader = null;

        try {
            reader = new FileReader("./input.txt");
        } catch (FileNotFoundException ex) {
            System.err.println("Could not find file... Error: " + ex.toString());
        }

        BufferedReader br = new BufferedReader(reader);
        ArrayList<String> lines = new ArrayList<String>();

        for (;;) {
            String line = null;
            try { 
                line = br.readLine();
            } catch (IOException ex) {
                System.err.println("Readline error... Error: " + ex.toString());
            }

            if (line == null) {
                break;
            }

            lines.add(line);
        }


        System.out.println("I have read the following lines: ");
        for (String s : lines) {
            System.out.println(s);
        }

        int[] nums = new int[lines.size()];

        for (int i = 0; i < lines.size(); ++i) {
            nums[i] = Integer.parseInt(lines.get(i));
        }

        // true for deeper, false for shallower
        boolean[] depths = new boolean[nums.length - 1];

        int curr = nums[0];
        for (int i = 1; i < nums.length; ++i) {
            boolean isDeeper = false; 
            if (nums[i] > curr) {
                isDeeper = true;
            }

            depths[i - 1] = isDeeper;

            curr = nums[i];
        }


        System.out.println(nums[0] + "(N/A - no previous measurement)");
        for (int i = 1; i < nums.length; ++i) {
            System.out.println(nums[i] + " (" + (depths[i - 1] ? "increased" : "decreased") + ")");
        }

        int count = 0;

        for (int i = 0; i < depths.length; ++i) {
            if (depths[i]) {
                count++;
            }
        }

        System.out.println("I counted " + count + " increased measurements.");
    }
}