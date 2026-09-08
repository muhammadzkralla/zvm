public class Main {
    public static void main(String[] args) {
        tableswitch(0);
        tableswitch(1);
        tableswitch(2);
        tableswitch(3);
        tableswitch(10);

        lookupswitch(0);
        lookupswitch(100);
        lookupswitch(200);
        lookupswitch(300);
        lookupswitch(1000);
    }

    public static void tableswitch(int value) {
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

    public static void lookupswitch(int value) {
        switch (value) {
            case 0:
                System.out.println("Zero");
                System.out.println("Zero");
                break;
            case 100:
                System.out.println("OneHundred");
                System.out.println("OneHundred");
                break;
            case 200:
                System.out.println("TwoHundred");
                System.out.println("TwoHundred");
                break;
            case 300:
                System.out.println("ThreeHundred");
                System.out.println("ThreeHundred");
                break;
            default:
                System.out.println("Other");
                System.out.println("Other");
                break;
        }
    }
}
