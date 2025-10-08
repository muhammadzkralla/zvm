public class Main {

    private static int num1 = 69;
    private static int num2 = 420;

    private static long lnum1 = 123456789012L;
    private static long lnum2 = 987654321098L;

    private static float fnum1 = 3.14f;
    private static float fnum2 = 2.5f;

    private static double dnum1 = 123.456789;
    private static double dnum2 = 987.654321;

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

        float faddition = fnum1 + fnum2;
        float fsubtraction = fnum2 - fnum1;
        float fmultiplication = fnum1 * fnum2;
        float fdivision = fnum2 / fnum1;
        float fremainder = fnum2 % fnum1;
        float fnegation = -fnum1;

        System.out.println(faddition);
        System.out.println(fsubtraction);
        System.out.println(fmultiplication);
        System.out.println(fdivision);
        System.out.println(fremainder);
        System.out.println(fnegation);

        double daddition = dnum1 + dnum2;
        double dsubtraction = dnum2 - dnum1;
        double dmultiplication = dnum1 * dnum2;
        double ddivision = dnum2 / dnum1;
        double dremainder = dnum2 % dnum1;
        double dnegation = -dnum1;

        System.out.println(daddition);
        System.out.println(dsubtraction);
        System.out.println(dmultiplication);
        System.out.println(ddivision);
        System.out.println(dremainder);
        System.out.println(dnegation);

        System.out.println("testing iinc instuction:");

        int num1 = 124124;
        int num2 = 127540;
        num1 += 100;
        num2 += -100;
        System.out.println(num1);
        System.out.println(num2);
    }
}
