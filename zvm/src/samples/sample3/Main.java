public class Main {

    private static int num1 = 69;
    private static int num2 = 420;

    private static long lnum1 = 123456789012L;
    private static long lnum2 = 987654321098L;

    public static void main(String[] args) {
        int addition = num1 + num2;
        int subtraction = num2 - num1;
        int multiplication = num1 * num2;
        int division = num2 / num1;

        int remainder = num2 % num1;
        int negation = -num1;

        System.out.println(addition);
        System.out.println(subtraction);
        System.out.println(multiplication);
        System.out.println(division);
        System.out.println(remainder);
        System.out.println(negation);

        long laddition = lnum1 + lnum2;
        long lsubtraction = lnum2 - lnum1;
        long lmultiplication = lnum1 * lnum2;
        long ldivision = lnum2 / lnum1;
        long lremainder = lnum2 % lnum1;
        long lnegation = -lnum1;

        System.out.println(laddition);
        System.out.println(lsubtraction);
        System.out.println(lmultiplication);
        System.out.println(ldivision);
        System.out.println(lremainder);
        System.out.println(lnegation);
    }
}
