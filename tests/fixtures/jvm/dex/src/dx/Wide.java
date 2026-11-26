package dx;

public class Wide {
    static double mix(long a, double b, int c, long d) {
        long t = a * d;
        double u = b * c;
        if (t > 100) {
            u += t;
        } else {
            t -= (long) u;
        }
        double v = u;
        long w = t;
        for (int k = 0; k < c; k++) {
            v = v * 1.5 + w;
            w = w * 3 + (long) v;
        }
        return v + w;
    }

    static long fib(int n) {
        long a = 0;
        long b = 1;
        for (int k = 0; k < n; k++) {
            long t = a + b;
            a = b;
            b = t;
        }
        return a;
    }

    static String reuse(int n) {
        String out = "";
        {
            long x = n * 1000000007L;
            out += x;
        }
        {
            double y = n / 3.0;
            out += "," + y;
        }
        {
            int z = n * n;
            float q = z / 2f;
            out += "," + z + "," + q;
        }
        return out;
    }

    public static void main(String[] args) {
        for (int k = 0; k < 6; k++) {
            System.out.println(mix(k * 7L, k / 4.0, k, 3L - k) + " " + fib(k * 10) + " " + reuse(k));
        }
        long big = Long.MAX_VALUE;
        big++;
        double nan = 0.0 / 0.0;
        System.out.println(big + " " + nan + " " + (nan == nan) + " " + Double.compare(nan, nan) + " " + (long) nan
                + " " + (int) Double.POSITIVE_INFINITY + " " + (long) -1e30);
    }
}
