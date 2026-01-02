public class Main {
    private static void foo(int num1, int num2) {
        System.out.println(num1);
        System.out.println(num2);
        String s = bar();
        System.out.println(s);
    }

    private static String bar() {
        return "Wait... WTF IT WORKED????? jhafkjhlskjhakjg";
    }

    public static void main(String[] args) {
        foo(69, 420);
        System.out.println(args[0]);
        System.out.println(args[1]);
    }
}
