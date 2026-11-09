package p;

public class HelloToy {
    public int twice(int x) {
        return x * 2;
    }

    public static class Inner {
        public int inc(int x) {
            return x + 1;
        }
    }
}
