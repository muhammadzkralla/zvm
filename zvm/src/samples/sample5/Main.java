class Main {
    static String returnString() {
        return "foo";
    }

    static double returnDouble() {
        return 3.14d;
    }

    static float returnFloat() {
        return 3.14f;
    }

    static int returnInt() {
        return 5;
    }

    static long returnLong() {
        return 123123123L;
    }

    public static void main(String[] args) {
        String s = returnString();
        double d = returnDouble();
        float f = returnFloat();
        int i = returnInt();
        long l = returnLong();
        System.out.println(s);
        System.out.println(d);
        System.out.println(f);
        System.out.println(i);
        System.out.println(l);
    }
}
