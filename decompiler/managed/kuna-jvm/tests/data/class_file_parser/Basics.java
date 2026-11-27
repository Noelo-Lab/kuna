import java.io.*;
import java.util.*;

public class Basics implements Serializable, Comparable<Basics> {
  public static final int I = 123456;
  public static final long L = 1234567890123L;
  public static final float F = 1.5f;
  public static final double D = 2.25;
  public static final String S = "const";
  public static final char C = 'x';
  public static final boolean Z = true;
  public static final byte B = -7;
  public static final short SH = 1000;
  protected transient int t;
  private volatile long v;
  int[] ints = new int[3];
  Object o;

  public int compareTo(Basics other) { return Integer.compare(t, other.t); }

  static int consts(int a) {
    int r = -1 + 0 + 1 + 2 + 3 + 4 + 5 + a;
    long l = 0L + 1L + a;
    float f = 0f + 1f + 2f + a;
    double d = 0d + 1d + a;
    r += 100; r += 1000; r += 100000;
    return r + (int) l + (int) f + (int) d;
  }

  static long conversions(int i, long l, float f, double d) {
    byte b = (byte) i; char c = (char) i; short s = (short) i;
    long a = i + (long) f + (long) d;
    float x = i + (float) l + (float) d;
    double y = i + (double) l + (double) f;
    int z = (int) l + (int) f + (int) d;
    return a + (long) x + (long) y + z + b + c + s;
  }

  static int compares(long a, long b, float c, float d, double e, double f, Object p, Object q) {
    int r = 0;
    if (a < b) r++;
    if (c < d) r++;
    if (c > d) r++;
    if (e < f) r++;
    if (e > f) r++;
    if (p == q) r++;
    if (p != q) r++;
    if (p == null) r++;
    if (q != null) r++;
    if (r == 1) r++; if (r != 2) r++; if (r < 3) r++; if (r >= 4) r++; if (r > 5) r++; if (r <= 6) r++;
    if (r == 0) r--; if (r != 0) r--; if (r < 0) r--; if (r >= 0) r--; if (r > 0) r--; if (r <= 0) r--;
    return r;
  }

  static long bits(int a, int b, long c, long d) {
    int x = (a << b) | (a >> b) ^ (a >>> b) & ~a;
    long y = (c << a) | (c >> a) ^ (c >>> a) & ~c & d;
    return x + y + (a % b) + (c % d) + (a / b) + (c / d) - a * b - c * d + (-a) + (-c);
  }

  static Object arrays(int n) {
    boolean[] z = new boolean[n]; char[] c = new char[n]; float[] f = new float[n];
    double[] d = new double[n]; byte[] b = new byte[n]; short[] s = new short[n];
    int[] i = new int[n]; long[] l = new long[n]; String[] str = new String[n];
    int[][] m = new int[n][n]; String[][][] m3 = new String[n][2][];
    z[0] = true; c[0] = 'a'; f[0] = 1; d[0] = 2; b[0] = 3; s[0] = 4; i[0] = 5; l[0] = 6; str[0] = "s";
    m[0][0] = i[0] + b[0] + s[0] + c[0] + (z[0] ? 1 : 0) + (int) f[0] + (int) d[0] + (int) l[0] + str.length;
    i[1] = i[0] = 7;
    l[1] = l[0] = 8;
    m3[0][0] = str;
    return m3;
  }

  static int strings(String s) {
    switch (s) {
      case "alpha": return 1;
      case "beta": return 2;
      case "Aa": return 3;
      case "BB": return 4;
      default: return 0;
    }
  }

  static int table(int x) {
    switch (x) {
      case 0: return 10; case 1: return 11; case 2: return 12; case 3: return 13;
      case 5: return 15; case 6: return 16; default: return -1;
    }
  }

  static int lookup(int x) {
    switch (x) {
      case -1000000: return 1; case 7: return 2; case 1 << 20: return 3; case Integer.MAX_VALUE: return 4;
      case Integer.MIN_VALUE: return 5; default: return 0;
    }
  }

  synchronized void sync() { t++; }

  void block() {
    synchronized (this) { t++; }
    synchronized (Basics.class) { v++; }
  }

  int tryCatch(InputStream in) throws IOException {
    try {
      return in.read();
    } catch (FileNotFoundException | EOFException e) {
      return -1;
    } catch (RuntimeException e) {
      throw new IOException(e);
    } finally {
      t++;
    }
  }

  void resources() throws Exception {
    try (InputStream a = new ByteArrayInputStream(new byte[1]); InputStream b = new ByteArrayInputStream(new byte[2])) {
      a.read(); b.read();
    }
  }

  static int varargs(String fmt, Object... args) { return args.length + fmt.length(); }

  long pops() { Long.hashCode(v); System.nanoTime(); Objects.hash(1, 2); return v; }

  int loops(int[] a) {
    int s = 0;
    for (int x : a) { if (x < 0) continue; if (x > 100) break; s += x; }
    int k = 0;
    do { k++; } while (k < 10);
    while (s > 1000) s /= 2;
    return s + k;
  }

  static boolean types(Object o) {
    if (o instanceof String) return ((String) o).isEmpty();
    if (o instanceof int[]) return ((int[]) o).length == 0;
    return o instanceof Runnable[][];
  }

  static { System.out.println(S + I); }
}
