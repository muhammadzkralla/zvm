public class Main {
    public static void main(String[] args) {
        testSwitch(0);
        testSwitch(1);
        testSwitch(2);
        testSwitch(3);
        testSwitch(10);
    }

    public static void testSwitch(int value) {
        switch (value) {
            case 0:
                System.out.println("Zero");
                System.out.println("Zero");
                break;
            case 1:
                System.out.println("One");
                System.out.println("One");
                break;
            case 2:
                System.out.println("Two");
                System.out.println("Two");
                break;
            case 3:
                System.out.println("Three");
                System.out.println("Three");
                break;
            default:
                System.out.println("Other");
                System.out.println("Other");
                break;
        }
    }
}
