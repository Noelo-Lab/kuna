package pkg;

import java.io.IOException;
import pkglib.TwrLock;
import pkglib.TwrStream;

public class TestTwrLibraryResource {
    private final TwrLock lock;
    private boolean flag;
    private int count;

    TestTwrLibraryResource(TwrLock lock) {
        this.lock = lock;
    }

    public void invalidate() {
        try (TwrLock.Held held = lock.write()) {
            clear();
            flag = false;
        }
    }

    public String name() {
        try (TwrLock.Held held = lock.write()) {
            clear();
            return "x";
        }
    }

    public int created() throws IOException {
        try (TwrStream s = new TwrStream()) {
            return s.read();
        }
    }

    public int two(String a, String b) throws IOException {
        try (TwrStream s = TwrStream.open(a); TwrStream t = TwrStream.open(b)) {
            return s.read() + t.read();
        }
    }

    public int caught(String a) {
        try (TwrStream s = TwrStream.open(a)) {
            return s.read();
        } catch (IOException e) {
            return -1;
        }
    }

    public void early(boolean b) {
        try (TwrLock.Held held = lock.write()) {
            if (b) {
                return;
            }
            clear();
        }
    }

    public void existing(TwrStream s) throws IOException {
        try (s) {
            count += s.read();
        }
    }

    public void nested(String a) throws IOException {
        try (TwrStream s = TwrStream.open(a)) {
            try (TwrLock.Held held = lock.write()) {
                count += s.read();
            }
        }
    }

    public void logsCloseFailure() {
        TwrLock.Held held = lock.write();
        try {
            clear();
        } catch (Throwable t) {
            if (held != null) {
                try {
                    held.close();
                } catch (Throwable t2) {
                    log(t2);
                }
            }
            throw t;
        }
        if (held != null) {
            held.close();
        }
    }

    public void returnSkipsClose(boolean b) {
        TwrLock.Held held = lock.write();
        try {
            if (b) {
                return;
            }
            clear();
        } catch (Throwable t) {
            if (held != null) {
                try {
                    held.close();
                } catch (Throwable t2) {
                    t.addSuppressed(t2);
                }
            }
            throw t;
        }
        if (held != null) {
            held.close();
        }
    }

    public void workBeforeTry() {
        TwrLock.Held held = lock.write();
        clear();
        try {
            flag = true;
        } catch (Throwable t) {
            if (held != null) {
                try {
                    held.close();
                } catch (Throwable t2) {
                    t.addSuppressed(t2);
                }
            }
            throw t;
        }
        if (held != null) {
            held.close();
        }
    }

    void clear() {
    }

    void log(Throwable t) {
    }
}
