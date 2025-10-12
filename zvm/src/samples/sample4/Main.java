class Main {
    public static void main(String[] args) {
        double d1 = 1.7976931348623157E308; // Very large (becomes Infinity)
        double d2 = 4.9E-324; // Very small (becomes 0.0)
        double d3 = Double.NaN; // NaN
        double d4 = -0.0; // Negative zero

        float f1 = (float) d1; // Infinity
        float f2 = (float) d2; // 0.0
        float f3 = (float) d3; // NaN
        float f4 = (float) d4; // -0.0

        System.out.println(f1);
        System.out.println(f2);
        System.out.println(f3);
        System.out.println(f4);
    }
}
