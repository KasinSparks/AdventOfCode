package days.day8.part2;

public enum SegmentType {
    A(1),
    B(2),
    C(4),
    D(8),
    E(16),
    F(32),
    G(64),
    UNK(128);

    private final int flag;

    private SegmentType(int flag) {
        this.flag = flag;
    }

    public int getFlag() { return this.flag; }

    public static SegmentType getSegmentType(int segment) {
        for (SegmentType t : SegmentType.values()) {
            if (t.getFlag() == segment) {
                return t;
            }
        }

        return SegmentType.UNK;
    }

    public static SegmentType getSegmentType(char segment) {
        SegmentType s;
        switch (segment) {
            case 'a': s = SegmentType.A;
                break;
            case 'b': s = SegmentType.B;
                break;
            case 'c': s = SegmentType.C;
                break;
            case 'd': s = SegmentType.D;
                break;
            case 'e': s = SegmentType.E;
                break;
            case 'f': s = SegmentType.F;
                break;
            case 'g': s = SegmentType.G;
                break;

            default: s = SegmentType.UNK;
        }

        return s;
    }
}
