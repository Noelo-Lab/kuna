package sx;

import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.Objects;
import java.util.Optional;
import java.util.Set;
import java.util.TreeSet;
import java.util.function.Function;

/** What d8 desugars (records, API backports, private lambda bodies) and the code shapes it moves. */
public class Sugar {
    record Point(int x, long y, String name, List<String> tags, boolean on, double w) {
        Point {
            if (x < 0) {
                throw new IllegalArgumentException("x");
            }
        }

        int twice() {
            return x * 2;
        }
    }

    record Box<T extends Comparable<T>>(T value, int[] cells) implements Comparable<Box<T>> {
        public int compareTo(Box<T> o) {
            return value.compareTo(o.value);
        }

        @Override
        public String toString() {
            return "Box" + value;
        }
    }

    record Empty() {}

    static final class Stats {
        final long a, b, c, d, e, f;

        Stats(long a, long b, long c, long d, long e, long f) {
            this.a = a;
            this.b = b;
            this.c = c;
            this.d = d;
            this.e = e;
            this.f = f;
        }

        Stats minus(Stats o) {
            return new Stats(
                    Math.max(0, a - o.a), Math.max(0, b - o.b), Math.max(0, c - o.c),
                    Math.max(0, d - o.d), Math.max(0, e - o.e), Math.max(0, f - o.f));
        }

        long sum() {
            return a + b + c + d + e + f;
        }
    }

    static class Base {
        final String label;

        Base(String label, int n) {
            this.label = label + n;
        }
    }

    static class Derived extends Base {
        Derived(String s, int k) {
            super(s.trim().toUpperCase(), k * 3 + s.length());
        }

        Derived(int k) {
            this(String.valueOf(k), k + 1);
        }
    }

    private int count;
    private static int secret = 5;

    /** Parameters updated in place or reassigned (d8 keeps them in their registers). */
    static final class Params {
        int size = 4;

        int successor(int i) {
            return ++i < size ? i : -1;
        }

        static int lastIndexOf(int[] a, int target, int from, int to) {
            for (to--; to >= from; to--) {
                if (a[to] == target) {
                    return to;
                }
            }
            return -1;
        }

        static long wide(long x, int k, double d) {
            x += k;
            x <<= 1;
            d *= 2;
            return x + (long) d;
        }

        static int onePath(int a, boolean c) {
            if (c) {
                a = a + 7;
            }
            return a * 3;
        }

        static int captured(int a) {
            final int before = a;
            a += 5;
            java.util.function.IntSupplier s = () -> before;
            return s.getAsInt() * 10 + a;
        }

        static int reassigned(int a, String s) {
            int sum = a;
            a = s.length();
            return sum + a + 1;
        }

        static String text(String s, int n) {
            while (n-- > 0) {
                s = s + n;
            }
            return s;
        }
    }

    private int hidden() {
        return count + 1;
    }

    static final class Peer {
        private final int v;

        private Peer(int v) {
            this.v = v;
        }

        int read(Sugar s) {
            secret++;
            s.count += v;
            return s.hidden() + s.count + v;
        }
    }

    private void add(String s) {
        count += s.length();
    }

    private boolean big(String s) {
        return s.length() > count;
    }

    int lambdas(List<String> xs) {
        xs.forEach(this::add);
        xs.forEach(s -> count += big(s) ? 2 : 1);
        Function<String, Integer> f = s -> s.length() + count;
        return f.apply("abc");
    }

    static boolean anyNegative(int[] v) {
        boolean found = false;
        for (int x : v) {
            found |= x < 0;
        }
        return found;
    }

    static boolean parses(String s) {
        boolean ok;
        try {
            Integer.parseInt(s);
            ok = true;
        } catch (NumberFormatException e) {
            ok = false;
        }
        return ok;
    }

    static int pick(boolean c, int a, int b) {
        return c ? a * 2 : b + 7;
    }

    static String backports(String s, int n) {
        StringBuilder out = new StringBuilder();
        out.append(List.of(s, "b", "c")).append(' ');
        out.append(new TreeSet<>(Set.of(3, 1, 2))).append(' ');
        out.append(Map.of("k", n).get("k")).append(' ');
        out.append(Objects.requireNonNullElse(null, "dflt")).append(' ');
        out.append("ab".repeat(n)).append(' ');
        out.append("  x  ".strip()).append(' ').append(" ".isBlank()).append(' ');
        out.append(Math.floorMod(-n, 5)).append(' ').append(Math.floorDiv(-7L, 2)).append(' ');
        out.append(Optional.empty().isEmpty()).append(' ');
        out.append(Objects.requireNonNullElseGet(null, () -> "got")).append(' ');
        out.append(Long.compare(n, 3)).append(' ').append(Integer.toUnsignedString(-1)).append(' ');
        out.append(List.copyOf(new ArrayList<>(List.of(n))));
        return out.toString();
    }

    public static void main(String[] args) {
        Point p = new Point(3, 4L, "p", List.of("t"), true, 1.5);
        Point q = new Point(3, 4L, "p", List.of("t"), true, 1.5);
        System.out.println(p + " " + p.equals(q) + " " + (p.hashCode() == q.hashCode()) + " " + p.twice());
        try {
            new Point(-1, 0, "", List.of(), false, 0);
        } catch (IllegalArgumentException e) {
            System.out.println("iae " + e.getMessage());
        }
        Box<String> b1 = new Box<>("a", new int[] {1});
        System.out.println(b1 + " " + b1.compareTo(new Box<>("b", new int[0])) + " " + new Empty() + " " + new Empty().equals(new Empty()));
        Stats s = new Stats(10, 20, 30, 40, 50, 60).minus(new Stats(1, 25, 3, 4, 5, 6));
        System.out.println(s.sum());
        System.out.println(new Derived(" ab ", 2).label + " " + new Derived(7).label);
        System.out.println(new Sugar().lambdas(List.of("a", "bb", "ccc")));
        System.out.println(anyNegative(new int[] {1, -2}) + " " + anyNegative(new int[] {1}) + " " + parses("12") + " " + parses("x"));
        System.out.println(pick(true, 3, 4) + " " + pick(false, 3, 4));
        System.out.println(backports("a", 2));
        System.out.println(new Peer(2).read(new Sugar()) + " " + secret);
        Params ps = new Params();
        System.out.println(ps.successor(1) + " " + ps.successor(3) + " " + Params.lastIndexOf(new int[] {1, 2, 1, 3}, 1, 0, 4)
                + " " + Params.wide(3L, 4, 1.5) + " " + Params.onePath(2, true) + " " + Params.onePath(2, false)
                + " " + Params.captured(4) + " " + Params.reassigned(6, "abc") + " " + Params.text("x", 3));
    }
}
