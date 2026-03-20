class Main {
    static int a = -1;
    static int x = 6;
    static int y = 5;
    static String nullStr = null;
    static String nonNullStr = "hello";

    public static void main(String[] args) {
        System.out.println("Testing null check instructions:");

        // IFNULL - branch if null
        if (nullStr == null) {
            System.out.println("nullStr is null");
        }

        // IFNONNULL - branch if not null
        if (nonNullStr != null) {
            System.out.println("nonNullStr is not null");
        }

        System.out.println("Testing multiple conditions:");

        // IFEQ
        if (a == 0) {
            System.out.println("a equals zero");
        }

        // IFNE
        if (a != 0) {
            System.out.println("a not equals zero");
        }

        // IFLT
        if (a < 0) {
            System.out.println("a less than zero");
        }

        // IFGE
        if (a >= 0) {
            System.out.println("a greater or equal zero");
        }

        // IFGT
        if (a > 0) {
            System.out.println("a greater than zero");
        }

        // IFLE
        if (a <= 0) {
            System.out.println("a less or equal zero");
        }

        System.out.println("Testing multiple if_icmp conditions:");

        // IF_ICMPEQ
        if (x == y) {
            System.out.println("x equals y");
        }

        // IF_ICMPNE
        if (x != y) {
            System.out.println("x not equals y");
        }

        // IF_ICMPLT
        if (x < y) {
            System.out.println("x less than y");
        }

        // IF_ICMPGE
        if (x >= y) {
            System.out.println("x greater or equal y");
        }

        // IF_ICMPGT
        if (x > y) {
            System.out.println("x greater than y");
        }

        // IF_ICMPLE
        if (x <= y) {
            System.out.println("x less or equal y");
        }
    }
}
