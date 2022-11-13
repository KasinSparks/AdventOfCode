package days.day10.part1;

import java.util.ArrayList;

public class ChunkManager {
    
    public ChunkManager() {}

    public Character validate(String line) {
        ArrayList<Chunk> chunkStack = new ArrayList<Chunk>();

        for (int i = 0; i < line.length(); ++i) {
            Character c = line.charAt(i);

            switch (c) {
                // New chunk
                case '(':
                case '[':
                case '{':
                case '<': 
                    if (chunkStack.size() == 0) {
                        chunkStack.add(new Chunk(ChunkType.getChunkType(c)));
                        break;
                    }

                    chunkStack.add(new Chunk(ChunkType.getChunkType(c)));

                    break;

                // Close chunk
                case ')':
                case ']':
                case '}':
                case '>':
                    if (chunkStack.size() < 1) {
                        break;
                    }

                    if (chunkStack.get(chunkStack.size() - 1).getChunkType() != 
                        ChunkType.getChunkType(c))
                    {
                        return c;     
                    }

                    chunkStack.remove(chunkStack.size() - 1);
                    break;

                default: break;
            }
        }

        return null;
    }
}
