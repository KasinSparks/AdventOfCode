package days.day6.part1;

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
        int newFishCount = 0;
        for (Lanternfish f : this._fish) {
            if (f.nextDay()) {
                newFishCount++;
            }
        }

        for (int i = 0; i < newFishCount; ++i) {
            this._fish.add(new Lanternfish());
        }
    }

    public int getFishCount() {
        return this._fish.size();
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
