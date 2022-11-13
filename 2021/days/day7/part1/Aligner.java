package days.day7.part1;

import java.util.ArrayList;

public class Aligner {
    private ArrayList<CrabSubmarine> _subs;

    public Aligner() {
        this._subs = new ArrayList<CrabSubmarine>();
    }

    public void addSub(CrabSubmarine sub) {
        this._subs.add(sub);
    }

    public int align() {
        // Brute force I guess
        int max = this.getMaxPosition();

        int minFuelCost = Integer.MAX_VALUE;
        //int minFuelPosition = 0;
        for (int i = 0; i < max; ++i) {
            int fuelCost = 0;
            for (CrabSubmarine sub : this._subs) {
                // Part1
                //int n = Math.abs(sub.getPosition() - i);
                //fuelCost += n;

                // Part2
                int n = Math.abs(sub.getPosition() - i);
                fuelCost += (n * (n + 1)) / 2;
            }

            if (fuelCost < minFuelCost) {
                minFuelCost = fuelCost;
                //minFuelPosition = i;
            }
        }

        return minFuelCost;
    }

    public int getMaxPosition() {
        int max = 0;

        for (CrabSubmarine sub : this._subs) {
            int pos = sub.getPosition();
            if (pos > max) {
                max = pos;
            }
        }

        return max;
    }
}
