package pkg;

public class TestSynchronizedLockAssignment {
    public interface Probe {
        void outside();

        void inside();
    }

    private final Probe probe;
    Object lock = new Object();
    Object[] slots = {new Object(), new Object()};
    int count;
    int calls;

    public TestSynchronizedLockAssignment(Probe probe) {
        this.probe = probe;
    }

    Object next() {
        calls++;
        return lock;
    }

    int swap() {
        Object l = this.lock;
        this.lock = new Object();
        synchronized (l) {
            count++;
            probe.inside();
        }
        return count;
    }

    int callBetween() {
        Object l = this.lock;
        probe.outside();
        synchronized (l) {
            count++;
            probe.inside();
        }
        return count;
    }

    int elementSwap(int i) {
        Object e = slots[i];
        slots[i] = new Object();
        synchronized (e) {
            count++;
            probe.inside();
        }
        return count;
    }

    int twoCalls() {
        Object l = next();
        synchronized (next()) {
            count++;
        }
        return calls;
    }

    int direct() {
        synchronized (this.lock) {
            this.lock = new Object();
            count++;
        }
        return count;
    }
}
