package days.day11.part1;

public class Octopus {
    private int _energyLevel;
    private boolean _isGlowing;

    public Octopus() {
        this._energyLevel = 0;
        this._isGlowing = false;
    }

    public Octopus(int energyLevel) {
        this._energyLevel = energyLevel;
        this._checkEnergyLevel();
    }

    public int getEnergyLevel() { return this._energyLevel; }
    public void setEnergyLevel(int energyLevel) {
        this._energyLevel = energyLevel;
        this._checkEnergyLevel();
    }

    public void incrementEnergyLevel() {
        this._energyLevel += 1;
        this._checkEnergyLevel();
    }
    
    public boolean isGlowing() { return this._isGlowing; }
    public void setIsGlowing(boolean isGlowing) {
        this._isGlowing = isGlowing;
        if (this._isGlowing) {
            this._energyLevel = 10;
        }
    }

    private void _checkEnergyLevel() {
        if (this._energyLevel >= 10) {
            this._isGlowing = true;
        } else {
            this._isGlowing = false;
        }
    }

    public void turnOff() {
        if (this._isGlowing) {
            this._isGlowing = false;
            this._energyLevel = 0;
        }
    }

    @Override
    public String toString() {
        StringBuilder sb = new StringBuilder();
        if (this._energyLevel > 9) {
            sb.append("\033[;32m");
            sb.append(this._energyLevel);
            sb.append("\033[0m");
        } else {
            sb.append(this._energyLevel);
        }
        return sb.toString();
    }
}
