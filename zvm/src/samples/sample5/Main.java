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

    public static void main(String[] args) {
        String s = returnString();
        double d = returnDouble();
        float f = returnFloat();
        System.out.println(s);
        System.out.println(d);
        System.out.println(f);
    }
}
