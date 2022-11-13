package days.day11.part2;

public class Octopus {
    private int _energyLevel;

    public Octopus() {
        this._energyLevel = 0;
    }

    public Octopus(int energyLevel) {
        this._energyLevel = energyLevel;
    }

    public int getEnergyLevel() { return this._energyLevel; }
    public void setEnergyLevel(int energyLevel) {
        this._energyLevel = energyLevel;
    }

    public void incrementEnergyLevel() {
        this._energyLevel += 1;
    }
    
    public boolean isGlowing() { return this._energyLevel > 9; }

    public void turnOff() {
		this._energyLevel = 0;
    }

    @Override
    public String toString() {
        StringBuilder sb = new StringBuilder();
        if (this._energyLevel > 9) {
            sb.append("g");
        } else {
            sb.append(this._energyLevel);
        }
        return sb.toString();
    }
}
