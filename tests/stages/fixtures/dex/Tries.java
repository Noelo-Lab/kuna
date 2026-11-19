// Source of tries.dex (with Base.java), the DEX fixture of
// tests/stages/kuna-dex-javatry.xml and kuna-dex-javactor.xml.
//
//   javac --release 8 -d classes Tries.java Base.java
//   java -cp r8-8.5.35.jar com.android.tools.r8.D8 --release --min-api 21 \
//       --output out classes/demo/*.class
//
// r8-8.5.35.jar is com.android.tools:r8:8.5.35 from Google's Maven repository.
package demo;

import java.io.IOException;

public class Tries extends Base {
    int field;

    public Tries(int x) {
        super(x, "t");
        field = x;
    }

    public Tries() {
        this(7);
    }

    static int parse(String s) {
        try {
            return Integer.parseInt(s);
        } catch (NumberFormatException e) {
            return -1;
        }
    }

    static int twoCatches(String s, int[] a) {
        int r = 0;
        try {
            r = Integer.parseInt(s);
            r += a[r];
        } catch (NumberFormatException e) {
            r = -1;
        } catch (ArrayIndexOutOfBoundsException e) {
            r = -2;
        }
        return r * 3;
    }

    static String withFinally(StringBuilder sb, String s) {
        String out = null;
        try {
            out = s.trim();
            sb.append(out);
        } finally {
            sb.append("!");
        }
        return out;
    }

    static void rethrow(String s) throws IOException {
        try {
            Integer.parseInt(s);
        } catch (RuntimeException e) {
            throw new IOException("bad", e);
        }
    }

    static int loopTry(int[] a) {
        int sum = 0;
        for (int i = 0; i < 10; i++) {
            try {
                sum += a[i];
            } catch (RuntimeException e) {
                sum--;
            }
        }
        return sum;
    }

    static Object sync(Object lock, StringBuilder sb) {
        synchronized (lock) {
            sb.append("x");
        }
        return lock;
    }
}
