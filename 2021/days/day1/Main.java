import java.util.ArrayList;
import java.io.BufferedReader;
import java.io.FileNotFoundException;
import java.io.FileReader;
import java.io.IOException;

public class Main {
    public static void main(String[] args) throws Exception {
        FileReader reader = null;

        try {
            reader = new FileReader("./input.txt");
            //reader = new FileReader("./sample_input.txt");
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

        int[] d = new int[nums.length - 1];

        if (nums.length < 3) {
            throw new Exception("Not enough input");
        }

        int[] sums = new int[nums.length - 2];

        curr = nums[0] + nums[1] + nums[2];
        sums[0] = curr;
        for (int i = 1; i < nums.length; ++i) {
            if (i + 2 >= nums.length) {
                break;
            }

            int sum = nums[i] + nums[i + 1] + nums[i + 2];
            sums[i] = sum;

            int depthMeasurement = 0;

            if (sum > curr) {
                depthMeasurement = 1; 
            } else if (sum < curr) {
                depthMeasurement = 2;
            }

            d[i - 1] = depthMeasurement;
            curr = sum;
        }

        count = 0;
        System.out.println(sums[0] + " (N/A - no previous sum)");
        for (int i = 1; i < sums.length; ++i) {
            System.out.print(sums[i] + " ");
            switch(d[i - 1]) {
                case 0:
                    System.out.println("(no change)");
                    break;
                case 1:
                    System.out.println("(increased)");
                    count++;
                    break;
                case 2:
                    System.out.println("(decreased)");
                    break;
                default:
                    break;
            }
        }

        System.out.println(count);
    }
}