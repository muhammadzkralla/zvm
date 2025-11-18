class Main {
    public static void main(String[] args) {
        int i1 = 1;
        int i2 = 0;
        long l1 = 1L;
        long l2 = 0L;

        int iand1 = i1 & i1;
        int iand2 = i1 & i2;
        int iand3 = i2 & i2;
        int iand4 = i2 & i1;

        int ior1 = i1 | i1;
        int ior2 = i1 | i2;
        int ior3 = i2 | i2;
        int ior4 = i2 | i1;

        int ixor1 = i1 ^ i1;
        int ixor2 = i1 ^ i2;
        int ixor3 = i2 ^ i2;
        int ixor4 = i2 ^ i1;

        long land1 = l1 & l1;
        long land2 = l1 & l2;
        long land3 = l2 & l2;
        long land4 = l2 & l1;

        long lor1 = l1 | l1;
        long lor2 = l1 | l2;
        long lor3 = l2 | l2;
        long lor4 = l2 | l1;

        long lxor1 = l1 ^ l1;
        long lxor2 = l1 ^ l2;
        long lxor3 = l2 ^ l2;
        long lxor4 = l2 ^ l1;

        System.out.println(iand1);
        System.out.println(iand2);
        System.out.println(iand3);
        System.out.println(iand4);

        System.out.println(ior1);
        System.out.println(ior2);
        System.out.println(ior3);
        System.out.println(ior4);

        System.out.println(ixor1);
        System.out.println(ixor2);
        System.out.println(ixor3);
        System.out.println(ixor4);

        System.out.println(land1);
        System.out.println(land2);
        System.out.println(land3);
        System.out.println(land4);

        System.out.println(lor1);
        System.out.println(lor2);
        System.out.println(lor3);
        System.out.println(lor4);

        System.out.println(lxor1);
        System.out.println(lxor2);
        System.out.println(lxor3);
        System.out.println(lxor4);

        // Test iand (bitwise AND)
        int ia = 0b1010; // 10
        int ib = 0b1100; // 12
        int iand = ia & ib; // 0b1000 = 8
        System.out.println(iand); // 8

        // Test ior (bitwise OR)
        int ior = ia | ib; // 0b1110 = 14
        System.out.println(ior); // 14

        // Test ixor (bitwise XOR)
        int ixor = ia ^ ib; // 0b0110 = 6
        System.out.println(ixor); // 6

        // Test land (bitwise AND for longs)
        long la = 0xFFFFFFFF00000000L;
        long lb = 0x00000000FFFFFFFFL;
        long land = la & lb; // 0
        System.out.println(land); // 0

        // Test lor (bitwise OR for longs)
        long lor = la | lb; // 0xFFFFFFFFFFFFFFFFL = -1
        System.out.println(lor); // -1

        // Test with negative numbers
        int neg1 = -5; // 0xFFFFFFFB in two's complement
        int neg2 = -3; // 0xFFFFFFFD in two's complement
        int negAnd = neg1 & neg2; // 0xFFFFFFF9 = -7
        System.out.println(negAnd); // -7

        // Test edge cases
        int allOnes = -1; // 0xFFFFFFFF
        int zero = 0;
        System.out.println(allOnes & zero); // 0
        System.out.println(allOnes | zero); // -1
    }
}
