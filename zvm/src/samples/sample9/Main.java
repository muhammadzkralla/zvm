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
