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

        // ------------------

        // f2i examples
        float fNaN = Float.NaN;
        float f37 = 3.7f;
        float fNeg37 = -3.7f;
        float fLarge = 1e20f;
        float fNegLarge = -1e20f;

        i1 = (int) fNaN; // 0 (uses f2i)
        i2 = (int) f37; // 3 (uses f2i)
        i3 = (int) fNeg37; // -3 (uses f2i)
        i4 = (int) fLarge; // i32::MAX (uses f2i)
        i5 = (int) fNegLarge; // i32::MIN (uses f2i)

        System.out.println(i1);
        System.out.println(i2);
        System.out.println(i3);
        System.out.println(i4);
        System.out.println(i5);

        // f2l examples
        float fNaN2 = Float.NaN;
        float f37_2 = 3.7f;
        float fVeryLarge = 1e20f;
        float fPosInf = Float.POSITIVE_INFINITY;

        l1 = (long) fNaN2; // 0L (uses f2l)
        l2 = (long) f37_2; // 3L (uses f2l)
        l3 = (long) fVeryLarge; // Large number (uses f2l)
        l4 = (long) fPosInf; // i64::MAX (uses f2l)

        System.out.println(l1);
        System.out.println(l2);
        System.out.println(l3);
        System.out.println(l4);

        // f2d examples
        f1 = 3.14f;
        f2 = Float.NaN;
        f3 = Float.POSITIVE_INFINITY;
        f4 = -0.0f;

        d1 = (double) f1; // 3.14... (uses f2d)
        d2 = (double) f2; // NaN (uses f2d)
        d3 = (double) f3; // Infinity (uses f2d)
        d4 = (double) f4; // -0.0 (uses f2d)

        System.out.println(d1);
        System.out.println(d2);
        System.out.println(d3);
        System.out.println(d4);

        // ------------------

        // l2i examples
        l1 = 123L;
        l2 = 2147483648L; // Larger than i32::MAX
        l3 = -1L;
        l4 = 0x123456789ABCDEFL; // Only low 32 bits kept

        i1 = (int) l1; // 123
        i2 = (int) l2; // -2147483648
        i3 = (int) l3; // -1
        i4 = (int) l4; // 0x9ABCDEF

        System.out.println(i1);
        System.out.println(i2);
        System.out.println(i3);
        System.out.println(i4);

        // l2f examples
        long lf1 = 123L;
        long lf2 = 16777217L; // Loses precision in float
        long lf3 = 9223372036854775807L; // Very large

        f1 = (float) lf1; // 123.0f
        f2 = (float) lf2; // 16777216.0f
        f3 = (float) lf3; // ~9.223372E18f

        System.out.println(f1);
        System.out.println(f2);
        System.out.println(f3);

        // l2d examples
        long ld1 = 123L;
        long ld2 = 9007199254740993L; // Loses precision in double
        long ld3 = 9223372036854775807L; // Very large

        d1 = (double) ld1; // 123.0
        d2 = (double) ld2; // ~9.007199254740992E15
        d3 = (double) ld3; // ~9.223372036854776E18

        System.out.println(d1);
        System.out.println(d2);
        System.out.println(d3);

        // ------------------

        // i2b examples
        int ib1 = 127;
        int ib2 = 128;
        int ib3 = 256;
        int ib4 = -1;

        byte b1 = (byte) ib1; // 127
        byte b2 = (byte) ib2; // -128
        byte b3 = (byte) ib3; // 0
        byte b4 = (byte) ib4; // -1

        System.out.println(b1);
        System.out.println(b2);
        System.out.println(b3);
        System.out.println(b4);

        // i2c examples
        int ic1 = 65;
        int ic2 = -1;
        int ic3 = 65536;
        int ic4 = 32768;

        char c1 = (char) ic1; // 65 'A'
        char c2 = (char) ic2; // 65535
        char c3 = (char) ic3; // 0
        char c4 = (char) ic4; // 32768

        System.out.println((int) c1);
        System.out.println((int) c2);
        System.out.println((int) c3);
        System.out.println((int) c4);

        // i2s examples
        int is1 = 32767;
        int is2 = 32768;
        int is3 = 65536;
        int is4 = -1;

        short s1 = (short) is1; // 32767
        short s2 = (short) is2; // -32768
        short s3 = (short) is3; // 0
        short s4 = (short) is4; // -1

        System.out.println(s1);
        System.out.println(s2);
        System.out.println(s3);
        System.out.println(s4);

        // i2l examples
        int il1 = 123;
        int il2 = -456;
        int il3 = 2147483647;

        l1 = (long) il1; // 123L
        l2 = (long) il2; // -456L
        l3 = (long) il3; // 2147483647L

        System.out.println(l1);
        System.out.println(l2);
        System.out.println(l3);

        // i2f examples
        int if1 = 123;
        int if2 = 16777217; // Loses precision

        f1 = (float) if1; // 123.0f
        f2 = (float) if2; // 16777216.0f

        System.out.println(f1);
        System.out.println(f2);

        // i2d examples
        int id1 = 123;
        int id2 = 2147483647;

        d1 = (double) id1; // 123.0
        d2 = (double) id2; // 2.147483647E9

        System.out.println(d1);
        System.out.println(d2);

    }
}
