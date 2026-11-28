package pkg;

public class TestFieldReadReuse {
    static class Lock {
        volatile int count = 1;
        int plain;
        Lock next;

        boolean tryInc(int i) {
            count = i + 1;
            return i > 3;
        }
    }

    Lock lock = new Lock();
    volatile Object value;
    int left;
    int right;

    int loop(Lock l) {
        int i;
        while ((i = l.count) > 0) {
            if (l.tryInc(i)) {
                return i;
            }
        }
        return -1;
    }

    int plainField(Lock l) {
        int p;
        if ((p = l.plain) > 2) {
            return p;
        }
        return 0;
    }

    Object cached() {
        Object local;
        return (local = value) == null ? (value = "made") : local;
    }

    Lock last() {
        Lock n;
        Lock end = lock;
        while ((n = end.next) != null) {
            end = n;
        }
        return end;
    }

    void both(Lock l) {
        left = right = l.plain;
    }

    void increment(Lock l) {
        l.plain++;
        int old = l.plain++;
        right = old;
    }
}
