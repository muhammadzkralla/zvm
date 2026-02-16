class Main {
    public static void main(String[] args) {
        // Create boolean array
        boolean[] boolArray = new boolean[2];
        System.out.println(boolArray.length);

        // Create char array
        char[] charArrray = new char[4];
        System.out.println(charArrray.length);

        // Create float array
        float[] floatArray = new float[3];
        System.out.println(floatArray.length);

        // Create double array
        double[] doubleArray = new double[3];
        System.out.println(doubleArray.length);

        // Create byte array
        byte[] byteArray = new byte[3];
        System.out.println(byteArray.length);

        // Create short array
        short[] shortArray = new short[3];
        System.out.println(shortArray.length);

        // Create int array
        int[] intArray = new int[5];
        System.out.println(intArray.length);

        // Create long array
        long[] longArray = new long[5];
        System.out.println(longArray.length);

        // Test array access
        intArray[0] = 42;
        System.out.println(intArray[0]);

        longArray[0] = 123123123L;
        System.out.println(longArray[0]);

        floatArray[1] = 3.14f;
        System.out.println(floatArray[1]);

        boolArray[0] = true;
        boolArray[1] = false;
        System.out.println(boolArray[0]);
        System.out.println(boolArray[1]);
    }
}
