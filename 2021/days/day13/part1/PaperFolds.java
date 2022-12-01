package days.day13.part1;

public class PaperFolds {
	private boolean _isHorizontal;
	private int _position;

	public PaperFolds(boolean isHorizontal, int position) {
		this._isHorizontal = isHorizontal;
		this._position = position;
	}

	public boolean getIsHorizontal() { return this._isHorizontal; }
	public int getPosition() { return this._position; }

	@Override
	public String toString() {
		return (this._isHorizontal ? "H" : "V") + " @ " + this._position;	
	}

}
