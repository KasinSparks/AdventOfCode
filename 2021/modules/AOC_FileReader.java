package modules;

import java.io.FileReader;
import java.util.ArrayList;
import java.io.FileNotFoundException;
import java.io.BufferedReader;
import java.io.IOException;


public class AOC_FileReader {
    private FileReader _reader;
    private BufferedReader _br;

    public AOC_FileReader(String filepath) {
        try {
            this._reader = new FileReader(filepath);
        } catch (FileNotFoundException ex) {
            System.err.println("Could not find file... Error: " + ex.toString());
        }

        this._br = new BufferedReader(this._reader);
    }

    public String readLine() throws IOException {
        return this._br.readLine();
    }

    public String[] readAllLines() {
        BufferedReader b = new BufferedReader(this._reader);
        ArrayList<String> lines = new ArrayList<String>();

        for (;;) {
            String line = null;
            try { 
                line = b.readLine();
            } catch (IOException ex) {
                System.err.println("Readline error... Error: " + ex.toString());
            }

            if (line == null) {
                break;
            }

            lines.add(line);
        }

        String[] l = new String[lines.size()];
        for (int i = 0; i < l.length; ++i) {
            l[i] = lines.get(i);
        } 

        return l;
    }
}