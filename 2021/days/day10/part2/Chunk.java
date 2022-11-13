package days.day10.part2;

import java.util.ArrayList;

public class Chunk {
    private ArrayList<Chunk> _subChunks;
    private boolean _hasClosed;
    private ChunkType _type;

    public Chunk(ChunkType type) {
        this._type = type;
        this._subChunks = new ArrayList<Chunk>();
        this._hasClosed = false;
    }

    public boolean getHasClosed() { return this._hasClosed; }
    public void setHasClosed(boolean isClosed) { this._hasClosed = isClosed; }

    public ChunkType getChunkType() { return this._type; }

    public ArrayList<Chunk> getSubChunks() { 
        if (this._subChunks.size() == 0) { 
            return null;
        }

        return this._subChunks;
    }

    public void addSubChunk(Chunk c) {
        this._subChunks.add(c);
    }
}
