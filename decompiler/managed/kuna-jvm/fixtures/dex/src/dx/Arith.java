package dx;

public class Arith {
    static int ii(int a, int b) {
        int r = a + b;
        r ^= a * b - (a | b) + (a & b);
        r += a << (b & 31);
        r -= a >> 3;
        r += a >>> 5;
        r += ~a;
        r += -b;
        return r;
    }

    static long ll(long a, long b, int s) {
        long r = a + b * 3 - (a ^ b);
        r += a << s;
        r -= a >> s;
        r ^= a >>> (s + 1);
        r += ~b;
        r |= a & 0xff00ff00ff00L;
        return r - (-a);
    }

    static String div(int a, int b) {
        try {
            return (a / b) + "," + (a % b);
        } catch (ArithmeticException e) {
            return "div0";
        }
    }

    static String ldiv(long a, long b) {
        try {
            return (a / b) + "," + (a % b);
        } catch (ArithmeticException e) {
            return "ldiv0";
        }
    }

    static String fp(float f, double d) {
        float g = f * 2.5f - f / 3f + f % 7f;
        double e = d * d - d / 9.0 + d % 2.0;
        return g + " " + e + " " + (float) e + " " + (double) g + " " + (-g) + " " + (-e);
    }

    static String compares(float f, double d) {
        StringBuilder sb = new StringBuilder();
        sb.append(f < 1f).append(f > 1f).append(f <= 1f).append(f >= 1f).append(f == 1f).append(f != 1f);
        sb.append(d < 1d).append(d > 1d).append(d <= 1d).append(d >= 1d).append(d == 1d).append(d != 1d);
        return sb.toString();
    }

    static String conv(long l, double d, float f, int i) {
        return (int) l + " " + (float) l + " " + (double) l + " " + (int) d + " " + (long) d + " "
                + (int) f + " " + (long) f + " " + (byte) i + " " + (char) (i & 0xffff) + " " + (short) i + " "
                + (long) i + " " + (float) i + " " + (double) i;
    }

    static int lits(int x) {
        int y = x + 1;
        y = y * 100;
        y = y / 7;
        y = y % 13;
        y = 1000 - y;
        y = 7 - y;
        y = y & 0x7f;
        y = y | 0x100;
        y = y ^ 0x55;
        y = y << 2;
        y = y >> 1;
        y = y >>> 1;
        y += 30000;
        y = y * -2;
        return y + (x / 3) + (x % 3);
    }

    static boolean bool(int a, int b) {
        boolean p = a > b;
        boolean q = a == b || a < 0;
        return p ^ q;
    }

    static int ternary(int a) {
        return a > 0 ? a * 2 : -a;
    }

    public static void main(String[] args) {
        int[] xs = {0, 1, -1, 7, -7, Integer.MAX_VALUE, Integer.MIN_VALUE, 12345};
        for (int a : xs) {
            for (int b : xs) {
                System.out.println(ii(a, b) + " " + div(a, b) + " " + bool(a, b));
            }
            System.out.println(lits(a) + " " + ternary(a));
        }
        long[] ls = {0L, 1L, -1L, Long.MAX_VALUE, Long.MIN_VALUE, 0x123456789L};
        for (long a : ls) {
            for (long b : ls) {
                System.out.println(ll(a, b, (int) (a & 63)) + " " + ldiv(a, b));
            }
        }
        float[] fs = {0f, -0f, 1f, -1.5f, Float.NaN, Float.POSITIVE_INFINITY, Float.MIN_VALUE, 3e38f};
        double[] ds = {0d, -0d, 1d, 2.75d, Double.NaN, Double.NEGATIVE_INFINITY, 1e300, -1e-300};
        for (int k = 0; k < fs.length; k++) {
            System.out.println(fp(fs[k], ds[k]) + " " + compares(fs[k], ds[k]));
            System.out.println(conv(ls[k % ls.length], ds[k], fs[k], xs[k]));
        }
        char c = 'A';
        c += 2;
        short s = (short) 40000;
        byte by = (byte) 200;
        System.out.println(c + " " + (int) c + " " + s + " " + by + " " + (s + by) + " " + (char) (c + 1));
    }
}
