package days.day8.part1;

public class Segment {
    public char _segmentLetter;
    public boolean _isLit;

    public Segment(char segmentLetter) throws Exception {
        if (!this._checkIfSegmentLetterIsValid(segmentLetter)) {
            throw new Exception("Segment letter [" + segmentLetter + "] was not in range of \'a\' to \'g\'");
        }

        this._segmentLetter = segmentLetter;
        this._isLit = false;
    }

    public void setIsLit(boolean lit) { this._isLit = lit; };
    public boolean getIsLit() { return this._isLit; }

    public void setSegmentLetter(char letter) {
        this._checkIfSegmentLetterIsValid(letter);
        this._segmentLetter = letter;
    }

    public char getSegmentLetter() { return this._segmentLetter; }


    private boolean _checkIfSegmentLetterIsValid(char letter) {
        char l = Character.toLowerCase(letter);  
        if (l >= 'a' || l <= 'g') {
            return true;
        }

        return false;
    }

    @Override
    public String toString() {
        return "Segment: " + this._segmentLetter + " -> " + (this._isLit ? "ON" : "OFF");
    }
}
