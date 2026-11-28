package pkg;

public class TestTwrClassVersion {
    interface Res extends AutoCloseable {
        @Override
        void close();

        int read();
    }

    private int total;

    Res open(int n) {
        return null;
    }

    public int first() {
        try (Res r = open(1)) {
            return r.read();
        }
    }

    public void both() {
        try (Res a = open(1); Res b = open(2)) {
            total = a.read() + b.read();
        }
    }
}
