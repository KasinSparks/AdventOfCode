package days.day10.part2;

public enum ChunkType {
    PARENTHESIS,
    SQUARE_BRACKET,
    CURLY_BRACE,
    ANGLE_BRACKET,
    UNK;

    public static ChunkType getChunkType(Character c) {
        ChunkType ct = UNK;

        switch (c) {
            case '(': case ')':
                ct = PARENTHESIS;
                break;
            case '[': case ']':
                ct = SQUARE_BRACKET;
                break;
            case '{': case '}':
                ct = CURLY_BRACE;
                break;
            case '<': case '>':
                ct = ANGLE_BRACKET;
                break;

            default: 
                ct = UNK;
                break;
        }

        return ct;
    }
}
