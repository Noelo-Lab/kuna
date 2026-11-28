package vx;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

// Register and constant sharing d8 writes that javac does not (debug and release, min-api 26):
// each method's comment names the shape and the benchmark method it was found in.
public class Shapes {
    enum Kind { OPEN, CLOSED }

    interface Source {
        boolean has();
        Object end();
        Kind kind();
    }

    static final class Fixed implements Source {
        final Object end;
        final Kind kind;
        Fixed(Object end, Kind kind) { this.end = end; this.kind = kind; }
        public boolean has() { return end != null; }
        public Object end() { return end; }
        public Kind kind() { return kind; }
    }

    // A constant 1 shared by a boolean return and `~i - 1` (Guava CharMatcher.RangesMatcher.matches).
    static boolean ranges(int[] starts, int[] ends, char c) {
        int index = Arrays.binarySearch(starts, c);
        if (index >= 0) {
            return true;
        }
        index = ~index - 1;
        return index >= 0 && c <= ends[index];
    }

    // `null` shared by values of two reference types (spring-boot ApplicationInfoPropertySource.readVersion).
    static String version(Class<?> c) {
        Package p = c != null ? c.getPackage() : null;
        return p != null ? p.getImplementationVersion() : null;
    }

    static final class Range<T extends Comparable<T>> {
        final T lower;
        final T upper;
        Range(T lower, T upper) { this.lower = lower; this.upper = upper; }
        boolean hasLower() { return lower != null; }
        boolean hasUpper() { return upper != null; }
        T lower() { return lower; }
        T upper() { return upper; }
        Kind lowerKind() { return Kind.CLOSED; }
        Kind upperKind() { return Kind.OPEN; }
    }

    // Temporaries of a type variable and of an enum in one register (Guava GeneralRange.from).
    static <T extends Comparable<T>> String from(Range<T> r) {
        T lower = r.hasLower() ? r.lower() : null;
        Kind lowerKind = r.hasLower() ? r.lowerKind() : Kind.OPEN;
        T upper = r.hasUpper() ? r.upper() : null;
        Kind upperKind = r.hasUpper() ? r.upperKind() : Kind.OPEN;
        return join(r.hasLower(), lower, lowerKind, r.hasUpper(), upper, upperKind);
    }

    static String join(boolean a, Object b, Kind c, boolean d, Object e, Kind f) {
        return a + " " + b + " " + c + " " + d + " " + e + " " + f;
    }

    // One register for two reference types of temporaries (Guava GeneralRange.from).
    static String bounds(Source r) {
        Object end = r.has() ? r.end() : null;
        Kind kind = r.has() ? r.kind() : Kind.OPEN;
        Object end2 = r.has() ? r.end() : null;
        Kind kind2 = r.has() ? r.kind() : Kind.CLOSED;
        return end + " " + kind + " " + end2 + " " + kind2;
    }

    // A constant used as an int and as the start of a loop counter, and 0 as false.
    static int counted(int[] v, boolean strict) {
        boolean seen = false;
        int n = 0;
        for (int i = 0; i < v.length; i++) {
            if (v[i] > 0 || !strict && v[i] == 0) {
                n++;
                seen = true;
            }
        }
        return seen ? n : -1;
    }

    // A constant flowing into a phi with a computed value, read in a handler.
    static int guarded(String s) {
        int r = 0;
        try {
            r = Integer.parseInt(s);
            r = r * 2;
        } catch (NumberFormatException e) {
            return r - 1;
        }
        return r;
    }

    // A boolean temporary and an int variable sharing a register.
    static int mixed(String a, List<String> list) {
        boolean empty = a.isEmpty();
        if (empty) {
            return 0;
        }
        int count = 0;
        for (String s : list) {
            if (s.startsWith(a) == !empty) {
                count += s.length();
            }
        }
        return count;
    }

    // A variable assigned on both arms before debug info starts it.
    static String arms(int k) {
        String x;
        if (k > 2) {
            x = "big";
        } else {
            x = "small";
        }
        StringBuilder sb = new StringBuilder(x);
        for (int i = 0; i < k; i++) {
            sb.append(i);
        }
        return sb.toString();
    }

    // A long constant and an int constant of the same bits.
    static long wide(long a, int b) {
        long r = 0L;
        if (a > 0) {
            r = a + 1;
        }
        return r + b * 1 + (a == 1 ? 1 : 0);
    }

    // A float 0 and an int 0 and a null.
    static String zeros(float f, int i, Object o) {
        float g = f > 0 ? f : 0f;
        int j = i > 0 ? i : 0;
        Object p = o != null ? o : null;
        return g + " " + j + " " + p;
    }

    enum Color { RED, GREEN, BLUE, GRAY }

    // d8 switches on ordinal() for an enum of the program, and DEX lists a class's fields by name.
    static String paint(Color c) {
        switch (c) {
            case RED:
                return "r";
            case GREEN:
                return "g";
            default:
                return "other";
        }
    }

    // d8's release build writes a small switch on a library enum as tests of its switch map
    // value (Guava DirectedGraphConnections.of).
    static String unit(java.util.concurrent.TimeUnit u) {
        switch (u) {
            case SECONDS:
                return "s";
            case MINUTES:
                return "m";
            default:
                return "?";
        }
    }

    static int seconds(java.util.concurrent.TimeUnit u) {
        switch (u) {
            case SECONDS:
                return 1;
            default:
                return 0;
        }
    }

    interface Fn<A, B> {
        B apply(A a);
    }

    int len(String x) {
        return x.length();
    }

    // A bound method reference d8 null-checks with Objects.requireNonNull right before it
    // (spring SimpleTypeReference.isPrimitive).
    static Fn<String, Integer> bound(Shapes s) {
        return s::len;
    }

    // An explicit check before a lambda capturing the same variable stays a statement.
    static Fn<String, Integer> checked(Shapes s) {
        java.util.Objects.requireNonNull(s);
        return x -> s.len(x) + 1;
    }

    // d8's release build drops the results of a string concatenation's appends.
    static String label(String name, int n) {
        return "item " + name + " #" + n;
    }

    public static void main(String[] args) {
        int[] starts = {1, 10, 20};
        int[] ends = {5, 15, 25};
        for (char c = 0; c < 30; c++) {
            System.out.print(ranges(starts, ends, c) ? '1' : '0');
        }
        System.out.println();
        System.out.println(version(null) + " " + version(Shapes.class));
        System.out.println(from(new Range<>("a", null)) + " | " + from(new Range<>(null, "b")));
        System.out.println(bounds(new Fixed(null, Kind.CLOSED)) + " | " + bounds(new Fixed("x", Kind.CLOSED)));
        System.out.println(counted(new int[] {0, 1, -1, 2}, true) + " " + counted(new int[] {0, 0}, false) + " " + counted(new int[0], true));
        System.out.println(guarded("21") + " " + guarded("x"));
        List<String> list = new ArrayList<>(Arrays.asList("ab", "abc", "b"));
        System.out.println(mixed("a", list) + " " + mixed("", list));
        System.out.println(arms(1) + " " + arms(4));
        System.out.println(wide(1, 2) + " " + wide(-3, 4));
        System.out.println(zeros(1.5f, -2, null) + " " + zeros(-1f, 3, "o"));
        for (Color c : Color.values()) {
            System.out.print(c.ordinal() + paint(c));
        }
        System.out.println();
        System.out.println(bound(new Shapes()).apply("abc") + " " + checked(new Shapes()).apply("ab"));
        try {
            bound(null);
        } catch (NullPointerException e) {
            System.out.println("npe");
        }
        System.out.println(label("a", 2));
        for (java.util.concurrent.TimeUnit u : java.util.concurrent.TimeUnit.values()) {
            System.out.print(unit(u) + seconds(u));
        }
        System.out.println();
    }
}
