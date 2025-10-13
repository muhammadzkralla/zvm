class Main {
    public static void main(String[] args) {

        // d2f examples
        double d1 = 1.7976931348623157E308; // Very large (becomes Infinity)
        double d2 = 4.9E-324; // Very small (becomes 0.0)
        double d3 = Double.NaN; // NaN
        double d4 = -0.0; // Negative zero
        double d5 = 3.4; // 3.4

        float f1 = (float) d1; // Infinity
        float f2 = (float) d2; // 0.0
        float f3 = (float) d3; // NaN
        float f4 = (float) d4; // -0.0
        float f5 = (float) d5; // 3.4

        System.out.println(f1);
        System.out.println(f2);
        System.out.println(f3);
        System.out.println(f4);
        System.out.println(f5);

        // d2i examples
        double dNaN = Double.NaN;
        double d37 = 3.7;
        double dNeg37 = -3.7;
        double dLarge = 1e20;
        double dNegLarge = -1e20;

        int i1 = (int) dNaN; // 0
        int i2 = (int) d37; // 3
        int i3 = (int) dNeg37; // -3
        int i4 = (int) dLarge; // 2147483647
        int i5 = (int) dNegLarge; // -2147483648

        System.out.println(i1);
        System.out.println(i2);
        System.out.println(i3);
        System.out.println(i4);
        System.out.println(i5);

        // d2l examples
        double dNaN2 = Double.NaN;
        double d37_2 = 3.7;
        double dVeryLarge = 1e100;
        double dPosInf = Double.POSITIVE_INFINITY;

        long l1 = (long) dNaN2; // 0L
        long l2 = (long) d37_2; // 3L
        long l3 = (long) dVeryLarge; // 9223372036854775807L
        long l4 = (long) dPosInf; // i64::MAX

        System.out.println(l1);
        System.out.println(l2);
        System.out.println(l3);
        System.out.println(l4);
    }
}
