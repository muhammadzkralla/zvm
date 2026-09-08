public class Main implements IMain {

    private static final String str = "lol";

    public static void foo(int num1, int num2) {
        System.out.println(num1);
        System.out.println(num2);
        bar();
    }

    public static void bar() {
        System.out.println("Wait... WTF IT WORKED????? jhafkjhlskjhakjg");
    }

    public static void main(String[] args) {
        foo(404, 501);
        System.out.println(args[0]);
        System.out.println(args[1]);
    }

    @Override
    public void hello() {
        System.out.println("Hello");
    }
}
