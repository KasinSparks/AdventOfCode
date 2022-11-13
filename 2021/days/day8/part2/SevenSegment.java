package days.day8.part2;

import days.day8.part2.Segment;

/*

 0:      1:      2:      3:      4:
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

  5:      6:      7:      8:      9:
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg
 
 */

/* 
| NUM | NUM OF SEGMENTS | UNIQUE |
|  0  |       6         |   n    |
|  1  |       2         |   y    |
|  2  |       5         |   n    |
|  3  |       5         |   n    |
|  4  |       4         |   y    |
|  5  |       5         |   n    |
|  6  |       6         |   n    |
|  7  |       3         |   y    |
|  8  |       7         |   y    |
|  9  |       6         |   n    |

 $
~  $
 ~ 
?  $
 ?
 */ 

public class SevenSegment {
    private static final int NUM_OF_SEGMENTS = 7;
    private Segment[] _segments;

    public SevenSegment() throws Exception {
        this._segments = new Segment[NUM_OF_SEGMENTS];
        SegmentType[] types = SegmentType.values();
        for (int i = 0; i < NUM_OF_SEGMENTS; ++i) {
            this._segments[i] = new Segment(types[i]);
        }
    }

    public void turnOnSegment(SegmentType segment) {
        for (Segment s : this._segments) {
            if (s.getSegmentType() == segment) {
                s.setIsLit(true);
                break;
            }
        }
    }

    public int getNumLit() {
        int sum = 0;
        for (Segment s : this._segments) {
            if (s.getIsLit()) { 
                sum++;
            }
        }
        return sum;
    }

    public SegmentType[] getLitSegmentsType() {
        SegmentType[] types = new SegmentType[this.getNumLit()];

        int count = 0;
        for (Segment s : this._segments) {
            if (s.getIsLit()) {
                types[count] = s.getSegmentType();
                count++;
            }
        }
        return types;
    }

    public int getLitStatus() {
        int curr = 0;
        for (Segment s : this._segments) {
            if (s.getIsLit()) { 
                curr |= s.getSegmentType().getFlag();
            }
        }

        return curr;
    }

    public boolean getLitStatus(SegmentType type) {
        for (Segment s : this._segments) {
            if (s.getSegmentType() == type) {
                return s.getIsLit();
            }
        }

        return false;
    }

    @Override
    public String toString() {
        StringBuilder sb = new StringBuilder();
        sb.append("[");
        for (Segment s : this._segments) {
            sb.append(s.getSegmentType() + ":" + s.getIsLit() + ", ");
        }
        sb.append("]");

        return sb.toString();
    }
}
