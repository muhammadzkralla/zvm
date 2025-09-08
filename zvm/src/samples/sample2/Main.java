class Main {
    static int a = -1;

    public static void main(String[] args) {
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
    }
}
