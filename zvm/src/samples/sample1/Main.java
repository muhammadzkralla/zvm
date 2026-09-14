public class Main implements IMain {

    private static final String str = "lol";

    public static void main(String[] args) {
        foo(32, 501);
        System.out.println(args[0]);
        System.out.println(args[1]);
    }

    public static void foo(int num1, int num2) {
        System.out.println(num1);
        System.out.println(num2);
        bar();
    }

    public static void bar() {
        System.out.println("Wait... WTF IT WORKED????? jhafkjhlskjhakjg");
    }

    @Override
    public void hello() {
        System.out.println("Hello");
    }
}
