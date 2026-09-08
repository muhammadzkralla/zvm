class Main {
    public static void main(String[] args) {
        // Test fcmpg and fcmpl
        float f1 = 3.0f;
        float f2 = 2.0f;
        float f3 = 3.0f;
        float fNaN = Float.NaN;

        System.out.println(compare(f1, f2)); // 1 (3.0 > 2.0)
        System.out.println(compare(f2, f1)); // -1 (2.0 < 3.0)
        System.out.println(compare(f1, f3)); // 0 (3.0 == 3.0)
        System.out.println(compareG(fNaN, f1)); // 1 (NaN with fcmpg)
        System.out.println(compareL(fNaN, f1)); // -1 (NaN with fcmpl)

        // Test dcmpg and dcmpl
        double d1 = 5.0;
        double d2 = 3.0;
        double d3 = 5.0;
        double dNaN = Double.NaN;

        System.out.println(compareD(d1, d2)); // 1 (5.0 > 3.0)
        System.out.println(compareD(d2, d1)); // -1 (3.0 < 5.0)
        System.out.println(compareD(d1, d3)); // 0 (5.0 == 5.0)
        System.out.println(compareDG(dNaN, d1)); // 1 (NaN with dcmpg)
        System.out.println(compareDL(dNaN, d1)); // -1 (NaN with dcmpl)

        System.out.println("=== Testing LCMP Instruction ===\n");

        // Test 1: value1 > value2 (should return 1)
        testLcmp(100L, 50L, "100 > 50");

        // Test 2: value1 < value2 (should return -1)
        testLcmp(50L, 100L, "50 < 100");

        // Test 3: value1 == value2 (should return 0)
        testLcmp(100L, 100L, "100 == 100");

        // Test 4: Positive vs negative
        testLcmp(100L, -50L, "100 > -50");
        testLcmp(-50L, 100L, "-50 < 100");

        // Test 5: Both negative
        testLcmp(-50L, -100L, "-50 > -100");
        testLcmp(-100L, -50L, "-100 < -50");

        // Test 6: Zero comparisons
        testLcmp(0L, 0L, "0 == 0");
        testLcmp(100L, 0L, "100 > 0");
        testLcmp(0L, 100L, "0 < 100");
        testLcmp(-100L, 0L, "-100 < 0");
        testLcmp(0L, -100L, "0 > -100");

        // Test 7: Large values
        testLcmp(Long.MAX_VALUE, Long.MIN_VALUE, "MAX > MIN");
        testLcmp(Long.MIN_VALUE, Long.MAX_VALUE, "MIN < MAX");
        testLcmp(Long.MAX_VALUE, Long.MAX_VALUE, "MAX == MAX");
        testLcmp(Long.MIN_VALUE, Long.MIN_VALUE, "MIN == MIN");

        // Test 8: Edge cases near boundaries
        testLcmp(Long.MAX_VALUE - 1, Long.MAX_VALUE, "(MAX-1) < MAX");
        testLcmp(Long.MIN_VALUE + 1, Long.MIN_VALUE, "(MIN+1) > MIN");

        System.out.println("\n=== All LCMP Tests Complete ===");
    }

    public static void testLcmp(long a, long b, String description) {
        // This will generate the lcmp instruction
        int result;
        if (a > b) {
            result = 1;
        } else if (a < b) {
            result = -1;
        } else {
            result = 0;
        }

        String resultStr;
        if (result == 1) {
            resultStr = "1 (greater)";
        } else if (result == -1) {
            resultStr = "-1 (less)";
        } else {
            resultStr = "0 (equal)";
        }

        System.out.print(description);
        System.out.print(" => ");
        System.out.print(resultStr);
        System.out.println("");
    }

    public static int compareLongs1(long a, long b) {
        // Direct comparison will generate lcmp
        if (a > b)
            return 1;
        if (a < b)
            return -1;
        return 0;
    }

    public static boolean isGreater(long a, long b) {
        // This generates: lcmp, ifle
        return a > b;
    }

    public static boolean isLess(long a, long b) {
        // This generates: lcmp, ifge
        return a < b;
    }

    public static boolean isEqual(long a, long b) {
        // This generates: lcmp, ifne
        return a == b;
    }

    public static boolean isNotEqual(long a, long b) {
        // This generates: lcmp, ifeq
        return a != b;
    }

    public static boolean isGreaterOrEqual(long a, long b) {
        // This generates: lcmp, iflt
        return a >= b;
    }

    public static boolean isLessOrEqual(long a, long b) {
        // This generates: lcmp, ifgt
        return a <= b;
    }

    static int compare(float a, float b) {
        // Uses fcmpg
        if (a > b)
            return 1;
        if (a == b)
            return 0;
        return -1;
    }

    // Always called with NaN
    static int compareG(float a, float b) {
        // Force fcmpg usage
        return (a > b) ? 1 : ((a == b) ? 0 : -1);
    }

    // Always called with NaN
    static int compareL(float a, float b) {
        // Force fcmpl usage
        return (a < b) ? -1 : ((a == b) ? 0 : 1);
    }

    static int compareD(double a, double b) {
        // Uses dcmpg
        if (a > b)
            return 1;
        if (a == b)
            return 0;
        return -1;
    }

    // Always called with NaN
    static int compareDG(double a, double b) {
        return (a > b) ? 1 : ((a == b) ? 0 : -1);
    }

    // Always called with NaN
    static int compareDL(double a, double b) {
        return (a < b) ? -1 : ((a == b) ? 0 : 1);
    }
}
