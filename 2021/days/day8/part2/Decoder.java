package days.day8.part2;

import days.day8.part2.Segment;

// We know 
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

     0
    aaaa
1  b    c 2
   b 3  c
    dddd
4  e    f 5
   e    f
    gggg
     6

We know c,f from the input 1
we know a from input 7 compared to 1

compare 4 and 1 to determine (b and d) or (d and b)
compare 8, 4, 7 to determine (e and g) or (e and g)

we know that 



 $
~ $
 ~ 
? $
 ?

*/ 

public class Decoder {
    private SevenSegment[] _segments;
    private Segment[] _decodedSegments;

    public Decoder(SevenSegment[] segments) {
        this._segments = segments;
        this.deducer();
    }

    private void deducer() {
        Segment[] segs = new Segment[7];

        // Find the one
        for (SevenSegment s : this._segments) {
            if (s.getNumLit() == 2) {
                // one
                // might have to switch these with each other later, but we need more numbers first
                segs[2] = new Segment(s.getLitSegmentsType()[0]);
                segs[5] = new Segment(s.getLitSegmentsType()[1]);
                break;
            }
        }

        //System.out.println(segs[2] + ", " + segs[5]);
        
        // Find seven
        for (SevenSegment s : this._segments) {
            if (s.getNumLit() == 3) {
                // seven
                if (segs[2] == null || segs[5] == null) {
                    // Cant do this yet
                    //throw new Exception("Could not find a one");
                }

                SegmentType[] types = s.getLitSegmentsType();
                //System.out.println(s);
                for (SegmentType t : types) {
                    boolean t0 = (t.getFlag() & segs[2].getSegmentType().getFlag()) == 0; 
                    boolean t1 = (t.getFlag() & segs[5].getSegmentType().getFlag()) == 0; 
                    if (t0 && t1) {
                        // unique
                        segs[0] = new Segment(t);
                        break;
                    }
                }
                break;
            }
        }
        //System.out.println(segs[0] + ", " + segs[2] + ", " + segs[5]);

        // Find four
        Segment[] wildcards = new Segment[2];
        for (SevenSegment s : this._segments) {
            if (s.getNumLit() == 4) {
                int count = 0;
                //System.out.println("Segment 4: " + s);

                SegmentType[] types = s.getLitSegmentsType();
                for (SegmentType t : types) {
                    boolean t0 = (t.getFlag() & segs[2].getSegmentType().getFlag()) == 0; 
                    boolean t1 = (t.getFlag() & segs[5].getSegmentType().getFlag()) == 0; 
                    if (t0 && t1) {
                        // unique
                        wildcards[count] = new Segment(t);
                        count++;
                    }
                }
                break;
            }
        }

        //System.out.println(wildcards[0]);
        //System.out.println(wildcards[1]);

        // Need to compare with zero to find segment #3 
        // We can also find zero since it wont have the middle segment
        // Which 4 does
        for (SevenSegment s : this._segments) {
            if (s.getNumLit() == 6) {
                SegmentType[] types = s.getLitSegmentsType();

                int val = 0;
                for (SegmentType t : types) {
                    val |= t.getFlag();
                }

                boolean t0 = (val & wildcards[0].getSegmentType().getFlag()) == 0; 
                boolean t1 = (val & wildcards[1].getSegmentType().getFlag()) == 0; 
                if (t0 ^ t1) {
                    // unique
                    if (t0) {
                        segs[3] = wildcards[0];
                        segs[1] = wildcards[1];
                    } else {
                        segs[1] = wildcards[0];
                        segs[3] = wildcards[1];
                    }
                    //break;
                }
                //break;
            }
        }
        //System.out.println(segs[0] + ", " + segs[1] + ", " + segs[2]
        //                    + ", " + segs[3] + ", " + segs[5]);

        // We just need segs 4 and 6 now
        // Find 9 or 3
        int almostNine = segs[0].getSegmentType().getFlag()
                        | segs[1].getSegmentType().getFlag()
                        | segs[2].getSegmentType().getFlag()
                        | segs[3].getSegmentType().getFlag()
                        | segs[5].getSegmentType().getFlag();

        for (SevenSegment s : this._segments) {
            if (s.getNumLit() == 6) {
                SegmentType[] types = s.getLitSegmentsType();
                int tempNum = 0;
                for (SegmentType t : types) {
                    tempNum |= t.getFlag();
                }

                if ((tempNum & almostNine) == almostNine) {
                    // found match
                    int val = tempNum ^ almostNine;
                    segs[6] = new Segment(SegmentType.getSegmentType(val));
                    break;
                }

            }
        }

        //System.out.println(segs[0] + ", " + segs[1] + ", " + segs[2]
        //                    + ", " + segs[3] + ", " + segs[5] + ", "
        //                    + segs[6]);


        // Deduce last segment
        int eight = SegmentType.A.getFlag()
                    | SegmentType.B.getFlag()
                    | SegmentType.C.getFlag()
                    | SegmentType.D.getFlag()
                    | SegmentType.E.getFlag()
                    | SegmentType.F.getFlag()
                    | SegmentType.G.getFlag();

        int tempNum = 0;
        for (Segment s : segs) {
            //System.out.println(s);
            if (s == null) {
                continue;
            }
            tempNum |= s.getSegmentType().getFlag();
        }

        int lastSegment = eight ^ tempNum;
        segs[4] = new Segment(SegmentType.getSegmentType(lastSegment));

        this._decodedSegments = segs;

        // Ensure the 2 and 5 segments are in the correct order
        try {
            this.decode();
        } catch (Exception ex) {
            // Segments were not in correct order
            System.err.println("Fixing order...");
            Segment temp = this._decodedSegments[2];
            this._decodedSegments[2] = this._decodedSegments[5];
            this._decodedSegments[5] = temp;
        }
    }

    public int[] decode() throws Exception {

        /*
         0
        1 2
         3
        4 5
         6
        */
        int[] nums = new int[10];

        //System.out.println(this._decodedSegments[3].getSegmentType().getFlag());

        nums[0] = this._decodedSegments[0].getSegmentType().getFlag()
                    | this._decodedSegments[1].getSegmentType().getFlag()
                    | this._decodedSegments[2].getSegmentType().getFlag()
                    | this._decodedSegments[4].getSegmentType().getFlag()
                    | this._decodedSegments[5].getSegmentType().getFlag()
                    | this._decodedSegments[6].getSegmentType().getFlag();

        nums[1] = this._decodedSegments[2].getSegmentType().getFlag()
                    | this._decodedSegments[5].getSegmentType().getFlag();

        nums[2] = this._decodedSegments[0].getSegmentType().getFlag()
             | this._decodedSegments[2].getSegmentType().getFlag()
             | this._decodedSegments[3].getSegmentType().getFlag()
             | this._decodedSegments[4].getSegmentType().getFlag()
             | this._decodedSegments[6].getSegmentType().getFlag();

        nums[3] = this._decodedSegments[0].getSegmentType().getFlag()
             | this._decodedSegments[2].getSegmentType().getFlag()
             | this._decodedSegments[3].getSegmentType().getFlag()
             | this._decodedSegments[5].getSegmentType().getFlag()
             | this._decodedSegments[6].getSegmentType().getFlag();

        nums[4] = this._decodedSegments[1].getSegmentType().getFlag()
             | this._decodedSegments[2].getSegmentType().getFlag()
             | this._decodedSegments[3].getSegmentType().getFlag()
             | this._decodedSegments[5].getSegmentType().getFlag();

        nums[5] = this._decodedSegments[0].getSegmentType().getFlag()
             | this._decodedSegments[1].getSegmentType().getFlag()
             | this._decodedSegments[3].getSegmentType().getFlag()
             | this._decodedSegments[5].getSegmentType().getFlag()
             | this._decodedSegments[6].getSegmentType().getFlag();

        nums[6] = this._decodedSegments[0].getSegmentType().getFlag()
             | this._decodedSegments[1].getSegmentType().getFlag()
             | this._decodedSegments[3].getSegmentType().getFlag()
             | this._decodedSegments[4].getSegmentType().getFlag()
             | this._decodedSegments[5].getSegmentType().getFlag()
             | this._decodedSegments[6].getSegmentType().getFlag();

        nums[7] = this._decodedSegments[0].getSegmentType().getFlag()
             | this._decodedSegments[2].getSegmentType().getFlag()
             | this._decodedSegments[5].getSegmentType().getFlag();

             
        nums[8] = this._decodedSegments[0].getSegmentType().getFlag()
             | this._decodedSegments[1].getSegmentType().getFlag()
             | this._decodedSegments[2].getSegmentType().getFlag()
             | this._decodedSegments[3].getSegmentType().getFlag()
             | this._decodedSegments[4].getSegmentType().getFlag()
             | this._decodedSegments[5].getSegmentType().getFlag()
             | this._decodedSegments[6].getSegmentType().getFlag();
             

        nums[9] = this._decodedSegments[0].getSegmentType().getFlag()
                    | this._decodedSegments[1].getSegmentType().getFlag()
                    | this._decodedSegments[2].getSegmentType().getFlag()
                    | this._decodedSegments[3].getSegmentType().getFlag()
                    | this._decodedSegments[5].getSegmentType().getFlag()
                    | this._decodedSegments[6].getSegmentType().getFlag();

        for (int i = 0; i < nums.length; ++i) {
            System.out.println("nums[" + i + "]: " + nums[i]);
        }

        int[] results = new int[this._segments.length];
        for (int i = 0; i < results.length; ++i) {
            results[i] = -1;
        }
        int counter = 0;

        
        for (SevenSegment s : this._segments) {
            System.out.println("Lit status: " + s.getLitStatus());
            boolean hasSlotted = false;
            for (int i = 0; i < nums.length; i++) {
                if (s.getLitStatus() == nums[i]) {
                    results[counter] = i;
                    hasSlotted = true;
                    //break;
                } else if (s.getLitStatus() == SegmentType.UNK.getFlag()) {
                    System.err.println("ERROR: found unknown type!");
                }
            }
            counter++;
            if (!hasSlotted) {
                System.err.println("ERROR: VALUE (" + s.getLitStatus() + ")DID NOT SLOT.");
                System.err.print("SEVEN SEGMENT: ");
                System.err.println(s);
                
                System.out.println((s.getLitStatus(this._decodedSegments[0].getSegmentType()) ? " **** " : ""));
                System.out.print((s.getLitStatus(this._decodedSegments[1].getSegmentType()) ? "*    " : "    "));
                System.out.println((s.getLitStatus(this._decodedSegments[2].getSegmentType()) ? "*" : ""));
                System.out.print((s.getLitStatus(this._decodedSegments[1].getSegmentType()) ? "*    " : "    "));
                System.out.println((s.getLitStatus(this._decodedSegments[2].getSegmentType()) ? "*" : ""));
                System.out.println((s.getLitStatus(this._decodedSegments[3].getSegmentType()) ? " **** " : ""));
                System.out.print((s.getLitStatus(this._decodedSegments[4].getSegmentType()) ? "*    " : "    "));
                System.out.println((s.getLitStatus(this._decodedSegments[5].getSegmentType()) ? "*" : ""));
                System.out.print((s.getLitStatus(this._decodedSegments[4].getSegmentType()) ? "*    " : "    "));
                System.out.println((s.getLitStatus(this._decodedSegments[5].getSegmentType()) ? "*" : ""));
                System.out.println((s.getLitStatus(this._decodedSegments[6].getSegmentType()) ? " **** " : ""));
                throw new Exception("ERROR: VALUE (" + s.getLitStatus() + ")DID NOT SLOT.");
            }
        }

        return results;
    }
}
