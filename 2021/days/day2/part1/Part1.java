import java.util.ArrayList;
import java.util.Scanner;
import java.io.BufferedReader;
import java.io.FileNotFoundException;
import java.io.FileReader;
import java.io.IOException;

public class Part1 {
    public static void main(String[] args) {
        FileReader reader = null;

        try {
            reader = new FileReader("./input.txt");
            //reader = new FileReader("./sample_input.txt");
        } catch (FileNotFoundException ex) {
            System.err.println("Could not find file... Error: " + ex.toString());
        }

        BufferedReader br = new BufferedReader(reader);
        ArrayList<String> lines = new ArrayList<String>();

        for (;;) {
            String line = null;
            try { 
                line = br.readLine();
            } catch (IOException ex) {
                System.err.println("Readline error... Error: " + ex.toString());
            }

            if (line == null) {
                break;
            }

            lines.add(line);
        }


        System.out.println("I have read the following lines: ");
        for (String s : lines) {
            System.out.println(s);
        }

        Commands commands = new Commands();

        for (String s : lines) {
            try {
                Command c = CommandParser.parse(s);
                switch (c.getCommandType()) {
                    case FORWARD:
                        commands.addHorizontal(c.getAmount());
                        break;
                    case DOWN:
                        commands.addDepth(c.getAmount());
                        break;
                    case UP:
                        commands.addDepth(-c.getAmount());
                        break;
                    default:
                        System.err.println("Unknown command: ");
                        break;

                }
            } catch (Exception ex) {
                System.err.println(ex);
            }

        }

        System.out.println("Final Position: " + commands.getFinalPosition());
    }
}