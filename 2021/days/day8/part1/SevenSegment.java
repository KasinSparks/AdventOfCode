package days.day8.part1;

import java.util.Dictionary;
import java.util.Enumeration;
import java.util.Hashtable;
import java.util.Iterator;
import java.util.Map;

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
 */ 

public class SevenSegment {
    private Hashtable<Character, Segment> _segments;

    public SevenSegment() throws Exception {
        try {
            this._segments = new Hashtable<Character, Segment>();

            this._segments.put('a', new Segment('a'));
            this._segments.put('b', new Segment('b'));
            this._segments.put('c', new Segment('c'));
            this._segments.put('d', new Segment('d'));
            this._segments.put('e', new Segment('e'));
            this._segments.put('f', new Segment('f'));
            this._segments.put('g', new Segment('g'));
        } catch (Exception ex) {
            throw ex;
        }
    }

    public void turnOnSegment(char segment) {
        this._segments.get(segment).setIsLit(true);
    }

    public int getNumLit() {
        int sum = 0;
        Enumeration<Segment> it = this._segments.elements();
        while(it.hasMoreElements()) {
            if (it.nextElement()._isLit) {
                sum++;
            }

        }
        return sum;
    }

    @Override
    public String toString() {
        StringBuilder sb = new StringBuilder();
        Enumeration<Character> it = this._segments.keys();
        sb.append("[");
        while (it.hasMoreElements()) {
            char c = it.nextElement();
            sb.append(c + ":" + this._segments.get(c)._isLit + ", ");
        }
        sb.append("]");

        return sb.toString();
    }
}
