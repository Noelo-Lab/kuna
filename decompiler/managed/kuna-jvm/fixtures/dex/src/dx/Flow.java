package dx;

import java.util.ArrayList;
import java.util.List;

public class Flow {
    private static final Object LOCK = new Object();
    private int counter;

    static String packed(int x) {
        switch (x) {
            case 0: return "zero";
            case 1: return "one";
            case 2:
            case 3: return "few";
            case 4: return "four";
            default: return "many";
        }
    }

    static int sparse(int x) {
        switch (x) {
            case -1000: return 1;
            case 7: return 2;
            case 100000: return 3;
            case Integer.MIN_VALUE: return 4;
            default: return 0;
        }
    }

    static int strings(String s) {
        switch (s) {
            case "alpha": return 1;
            case "beta": return 2;
            case "gamma": return 3;
            default: return -1;
        }
    }

    static String nested(int mode) {
        StringBuilder log = new StringBuilder();
        try {
            log.append('a');
            try {
                log.append('b');
                if (mode == 1) {
                    throw new IllegalStateException("one");
                }
                if (mode == 2) {
                    throw new UnsupportedOperationException("two");
                }
                log.append('c');
            } catch (IllegalStateException e) {
                log.append("I:").append(e.getMessage());
                if (mode == 1) {
                    throw new RuntimeException("re", e);
                }
            } finally {
                log.append('f');
            }
            log.append('d');
        } catch (RuntimeException e) {
            log.append("R:").append(e.getMessage());
        } finally {
            log.append('F');
        }
        return log.toString();
    }

    static int finallyReturn(int x) {
        int y = x;
        try {
            if (x > 3) {
                return y * 2;
            }
            y += 10;
        } finally {
            y = -1;
        }
        return y;
    }

    synchronized void bump() {
        counter++;
    }

    int locked(int n) {
        synchronized (LOCK) {
            for (int k = 0; k < n; k++) {
                bump();
            }
            if (n > 100) {
                throw new IllegalArgumentException();
            }
        }
        return counter;
    }

    static int loops(int n) {
        int total = 0;
        outer:
        for (int a = 0; a < n; a++) {
            int b = 0;
            while (true) {
                b++;
                if (b > a) {
                    continue outer;
                }
                if (a * b > 20) {
                    break outer;
                }
                total += a * b;
            }
        }
        int k = n;
        do {
            total ^= k;
            k -= 3;
        } while (k > 0);
        return total;
    }

    static String nulls(Object o, String s) {
        if (o == null) {
            return "null";
        }
        if (o == s) {
            return "same";
        }
        if (o instanceof String && ((String) o).length() > 2) {
            return "str:" + o;
        }
        if (o instanceof Integer) {
            Integer i = (Integer) o;
            return "int:" + (i + 1);
        }
        return "obj:" + o.getClass().getSimpleName();
    }

    public static void main(String[] args) {
        for (int k = -2; k < 7; k++) {
            System.out.println(packed(k) + " " + sparse(k) + " " + loops(k));
        }
        System.out.println(sparse(-1000) + sparse(100000) + sparse(Integer.MIN_VALUE));
        for (String s : new String[] {"alpha", "beta", "gamma", "delta"}) {
            System.out.println(strings(s));
        }
        for (int m = 0; m < 3; m++) {
            System.out.println(nested(m) + " " + finallyReturn(m * 2));
        }
        Flow f = new Flow();
        System.out.println(f.locked(5));
        try {
            f.locked(101);
        } catch (IllegalArgumentException e) {
            System.out.println("iae " + f.counter);
        }
        String s = "abc";
        List<Object> objs = new ArrayList<>();
        objs.add(null);
        objs.add(s);
        objs.add("xy");
        objs.add("xyz");
        objs.add(41);
        objs.add(2.5);
        for (Object o : objs) {
            System.out.println(nulls(o, s));
        }
    }
}
