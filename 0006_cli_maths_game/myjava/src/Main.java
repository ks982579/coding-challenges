import java.util.Random;

// Press Shift twice to open the Search Everywhere dialog and type `show whitespaces`,
// then press Enter. You can now see whitespace characters in your code.
public class Main {
    public static void main(String[] args) {
        // Press Alt+Enter with your caret at the highlighted text to see how
        // IntelliJ IDEA suggests fixing it.
        System.out.printf("Hello and welcome!");
        Random rand = new Random();

        // Just use shifter to add or subtract values
        int shifter = 0;
        System.out.println(rand.nextInt(10));

        // Press Shift+F10 or click the green arrow button in the gutter to run the code.
        for (int i = 1; i <= 5; i++) {

            // Press Shift+F9 to start debugging your code. We have set one breakpoint
            // for you, but you can always add more by pressing Ctrl+F8.
            System.out.println("i = " + i);
        }
    }
}

class GameLoop {
    private int shifter = 0;
    private int start = 0;
    private int end = 10;

    public int getShifter() {
        return shifter;
    }

    public void setShifter(int shifter) {
        this.shifter = shifter;
    }
}