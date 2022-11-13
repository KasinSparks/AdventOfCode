import javax.swing.text.html.HTMLDocument.HTMLReader.SpecialAction;

public class CommandParser {
    private CommandParser() {}

    public static Command parse(String s) throws Exception {
        int spacePosition = -1;
        for (int i = 0; i < s.length(); ++i) {
            if (s.charAt(i) == ' ') {
                spacePosition = i;
                break;
            }
        }

        if (spacePosition == -1) {
            // Error
            throw new Exception("No space found in command.");
        }

        String command = s.substring(0, spacePosition);
        String amountStr = s.substring(spacePosition + 1, s.length());
        int amount = Integer.parseInt(amountStr);

        CommandType t;

        System.out.println(command);

        if (command.equals("forward")) {
            t = CommandType.FORWARD;
        } else if (command.equals("down")) {
            t = CommandType.DOWN;
        } else if (command.equals("up")) {
            t = CommandType.UP;
        } else {
            throw new Exception("Unknown command: " + command);
        }

        Command c = new Command(t, amount);

        return c;
    }
}
