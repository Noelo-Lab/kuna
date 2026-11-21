package pkg;

public class TestTwrDesugaringShape {
    interface Res extends AutoCloseable {
        @Override
        void close();

        int read();
    }

    private final Res fixed;
    private Res current;
    int count;
    boolean failing;
    String trace = "";

    TestTwrDesugaringShape(Res fixed) {
        this.fixed = fixed;
        this.current = fixed;
    }

    Res open() {
        trace += "open;";
        return current;
    }

    void work() {
        trace += "work;";
        if (failing) {
            throw new RuntimeException("work");
        }
        count++;
    }

    public int real() {
        try (Res r = open()) {
            return r.read();
        }
    }

    public void finalField() {
        try (fixed) {
            count = fixed.read();
        }
    }

    public void plainField() {
        try (Res r = current) {
            work();
        }
    }

    public void logsCloseFailure() {
        Res r = open();
        try {
            work();
        } catch (Throwable t) {
            if (r != null) {
                try {
                    r.close();
                } catch (Throwable t2) {
                    log(t2);
                }
            }
            throw t;
        }
        if (r != null) {
            r.close();
        }
    }

    public void returnSkipsClose(boolean b) {
        Res r = open();
        try {
            if (b) {
                return;
            }
            work();
        } catch (Throwable t) {
            if (r != null) {
                try {
                    r.close();
                } catch (Throwable t2) {
                    t.addSuppressed(t2);
                }
            }
            throw t;
        }
        if (r != null) {
            r.close();
        }
    }

    public void workBeforeTry() {
        Res r = open();
        work();
        try {
            work();
        } catch (Throwable t) {
            if (r != null) {
                try {
                    r.close();
                } catch (Throwable t2) {
                    t.addSuppressed(t2);
                }
            }
            throw t;
        }
        if (r != null) {
            r.close();
        }
    }

    public void rethrowsWrapped() {
        Res r = open();
        try {
            work();
        } catch (Throwable t) {
            if (r != null) {
                try {
                    r.close();
                } catch (Throwable t2) {
                    t.addSuppressed(t2);
                }
            }
            throw new IllegalStateException(t);
        }
        if (r != null) {
            r.close();
        }
    }

    public void closesUnchecked() {
        Res r = open();
        try {
            work();
        } catch (Throwable t) {
            try {
                r.close();
            } catch (Throwable t2) {
                t.addSuppressed(t2);
            }
            throw t;
        }
        r.close();
    }

    void log(Throwable t) {
        trace += "log;";
    }
}
