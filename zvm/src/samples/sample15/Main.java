public class Main {
    public static void main(String[] args) {
        Counter counter = new Counter();
        counter.increment();
        int count = counter.getCount();
        System.out.println(count);
    }
}

class Counter {
    public static int count = 0;

    public void increment() {
        count++;
    }

    public int getCount() {
        return count;
    }
}
