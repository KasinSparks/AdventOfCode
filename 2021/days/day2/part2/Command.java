public class Command {
    private CommandType commandType;
    private int amount;
    
    public Command(CommandType t, int n) {
        this.commandType = t;
        this.amount = n;
    }

    public CommandType getCommandType() { return this.commandType; }
    public int getAmount() { return this.amount; }
}
