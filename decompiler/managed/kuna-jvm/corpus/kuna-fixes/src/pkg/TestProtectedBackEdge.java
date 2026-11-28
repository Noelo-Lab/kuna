package pkg;

public class TestProtectedBackEdge {
    boolean running;
    int runs;
    int count;
    int left;
    final Object lock = new Object();

    void returnInLoop() {
        try {
            while (true) {
                synchronized (lock) {
                    if (count <= 0) {
                        return;
                    }
                }
                count--;
            }
        } finally {
            runs++;
        }
    }

    void breakInLoop() {
        try {
            while (true) {
                synchronized (lock) {
                    if (count <= 0) {
                        break;
                    }
                }
                count--;
            }
            left += 2;
        } finally {
            runs++;
        }
    }

    void nestedLoop() {
        try {
            while (true) {
                synchronized (lock) {
                    while (count > 2) {
                        if (left <= 0) {
                            return;
                        }
                        left--;
                        count--;
                    }
                }
                count += 3;
            }
        } finally {
            runs++;
        }
    }

    void worker() {
        boolean flagged = false;
        boolean started = false;
        try {
            while (true) {
                synchronized (lock) {
                    if (!started) {
                        if (running) {
                            return;
                        } else {
                            running = true;
                            started = true;
                        }
                    }
                    if (left == 0) {
                        running = false;
                        return;
                    }
                    left--;
                }
                flagged |= count > 3;
                try {
                    count++;
                } finally {
                    count += 2;
                }
            }
        } finally {
            runs++;
            if (flagged) {
                count = -count;
            }
        }
    }

    void tryInLoop() {
        while (true) {
            try {
                synchronized (lock) {
                    if (count <= 0) {
                        return;
                    }
                }
                count--;
            } finally {
                runs++;
            }
        }
    }
}
