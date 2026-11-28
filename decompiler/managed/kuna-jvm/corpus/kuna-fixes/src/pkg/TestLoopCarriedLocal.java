package pkg;

public class TestLoopCarriedLocal {
    private int seed = 17;

    int step(int k) {
        seed = seed * 31 + k;
        return seed;
    }

    long draw(long bound) {
        long acc;
        long rem;
        do {
            acc = ((long) step(3)) << 20;
            acc ^= step(5) & 0xfffL;
            rem = acc % bound;
        } while (acc - rem < 0);
        return rem;
    }

    double settle(double x) {
        int n = (int) x;
        double head;
        double tail;
        while (true) {
            double t = n * 0.25;
            head = x - t;
            tail = head * t;
            t = n * 0.5;
            double u = head;
            head = t + u;
            tail += head - u;
            if (head > 0) {
                break;
            }
            --n;
        }
        return head + tail;
    }
}
