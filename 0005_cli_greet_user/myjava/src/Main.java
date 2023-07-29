import java.util.Scanner;

// Press Shift twice to open the Search Everywhere dialog and type `show whitespaces`,
// then press Enter. You can now see whitespace characters in your code.
public class Main {
    public static void main(String[] args) {
        // Press Alt+Enter with your caret at the highlighted text to see how
        // IntelliJ IDEA suggests fixing it.
        System.out.println("Hello and welcome!");

        // Press Shift+F10 or click the green arrow button in the gutter to run the code.
        String userName = getName();
        int userAge = getAge();
        System.out.println("Hello " + userName+".");
        System.out.println("I heard you are " + userAge + " years young.");
    }

    private static String getName() {
        Scanner userNameGetter = new Scanner(System.in);
        System.out.print("Please enter your name: ");
        return userNameGetter.nextLine();
    }

    private static int getAge() {
        Scanner userAgeGetter = new Scanner(System.in);
        System.out.print("Please enter your age: ");
        String userAnswer = userAgeGetter.nextLine();
        // String to int
        int userAge = Integer.parseInt(userAnswer);
        return userAge;
    }
}

