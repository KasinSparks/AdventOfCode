package days.day8.part2;

public class Segment {
    private SegmentType _segmentLetter;
    private boolean _isLit;


    public Segment(SegmentType segmentLetter) {
        this._segmentLetter = segmentLetter;
        this._isLit = false;
    }

    public void setIsLit(boolean lit) { this._isLit = lit; };
    public boolean getIsLit() { return this._isLit; }

    public SegmentType getSegmentType() { return this._segmentLetter; }
    public void setSegmentLetter(SegmentType letter) {
        this._segmentLetter = letter;
    }

    public SegmentType pgetSegmentLetter() { return this._segmentLetter; }


    @Override
    public String toString() {
        return "Segment: " + this._segmentLetter + " -> " + (this._isLit ? "ON" : "OFF");
    }
}
