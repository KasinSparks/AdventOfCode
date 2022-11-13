package days.day6.part2;

import java.util.ArrayList;

public class FishPool {
    private ArrayList<Lanternfish> _fish;

    public FishPool() {
        this._fish = new ArrayList<Lanternfish>();
    }
    
    public FishPool(Lanternfish[] fishes) {
        this._fish = new ArrayList<Lanternfish>();

        // May need a deep copy
        for (Lanternfish f : fishes) {
            this._fish.add(f);
        }
    }

    public void nextDay() {
        long newFishCount = 0;
        for (Lanternfish f : this._fish) {
            if (f.nextDay()) {
                newFishCount += f.getModifier();
            }
        }

        this._fish.add(new Lanternfish(newFishCount));
    }

    public long getFishCount() {
        long count = 0;

        for (Lanternfish f : this._fish) {
            count += f.getModifier();
        }

        return count;
    }

    @Override
    public String toString() {
        StringBuilder sb = new StringBuilder();

        for (Lanternfish f : this._fish) {
            sb.append(f + ",");
        }

        return sb.toString();
    }
}
