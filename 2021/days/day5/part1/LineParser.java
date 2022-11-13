package day5.part1;

public class LineParser {
    public static Point[] Parse(String line) {
        Point[] points = new Point[2];

        int[] tempVals = new int[4];

        int j = 0;
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < line.length(); ++i) {
            char c = line.charAt(i);
            if (c == ',') {
                // end current number and start new number
                tempVals[j] = Integer.parseInt(sb.toString());
                sb = new StringBuilder();
                j++;
                continue;
            }

            // encoutered first space
            if (c == ' ') {
                // end current number, and skip to next number
                tempVals[j] = Integer.parseInt(sb.toString());
                tempVals[j] = Integer.parseInt(sb.toString());
                sb = new StringBuilder();
                j++;
                i += 3;
                continue;
            }

            sb.append(c);

            if (i >= line.length() - 1) {
                // end current number and start new number
                tempVals[j] = Integer.parseInt(sb.toString());
            }
        }

        for (int i = 0; i < tempVals.length; i += 2) {
            points[i / 2] = new Point(tempVals[i], tempVals[i + 1]);
        }

        return points;
    }    
}
