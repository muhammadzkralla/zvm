class Main {
    static String foo() {
        return "foo";
    }

    static double bar() {
        return 3.14d;
    }

    public static void main(String[] args) {
        String s = foo();
        double d = bar();
        System.out.println(s);
        System.out.println(d);
    }
}
