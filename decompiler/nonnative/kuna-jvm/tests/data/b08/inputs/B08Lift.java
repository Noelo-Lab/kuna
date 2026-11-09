// Fixture for tests/b08_lift.rs: method bodies covering the opcode groups of
// ExprProcessor.processBlock, mostly without invocations.
public class B08Lift {
  static int si;
  static long sl;
  static double sd;
  static Object so;
  int fi;
  long fl;
  float ff;
  double fd;
  Object fo;

  static int arith(int a, int b, long c, long d, float e, float f, double g, double h) {
    int r = a + b - a * b / (b | 1) % 7;
    long s = c + d - c * d / (d | 1L) % 7L;
    float t = e + f - e * f / f % 3f;
    double u = g + h - g * h / h % 3.0;
    r = r << 2 >> 1 >>> 3 & 0xff | 0x10 ^ a;
    s = s << 2 >> 1 >>> 3 & 0xffL | 0x10L ^ c;
    return r + (int) s + (int) t + (int) u + -a;
  }

  static double negs(long a, float b, double c, int d) {
    return -a + -b + -c + -d;
  }

  static double conv(int i, long l, float f, double d) {
    byte b = (byte) i;
    char ch = (char) i;
    short sh = (short) i;
    long a = i;
    float x = i;
    double y = i;
    int j = (int) l;
    float lf = l;
    double ld = l;
    int fi = (int) f;
    long fl = (long) f;
    double fd = f;
    int di = (int) d;
    long dl = (long) d;
    float df = (float) d;
    return b + ch + sh + a + x + y + j + lf + ld + fi + fl + fd + di + dl + df;
  }

  static int cmp(long a, long b, float c, float d, double e, double f, Object o, Object p) {
    int r = 0;
    if (a < b) r++;
    if (c > d) r++;
    if (c < d) r++;
    if (e <= f) r++;
    if (e >= f) r++;
    if (r == 0) r += 2;
    if (r != 3) r += 3;
    if (r < 4) r += 4;
    if (r >= 5) r += 5;
    if (r > 6) r += 6;
    if (r <= 7) r += 7;
    if (r == a) r += 8;
    if (r != (int) b) r--;
    if (o == null) r--;
    if (o != null) r--;
    if (o == p) r--;
    if (o != p) r--;
    return r;
  }

  static long consts() {
    long a = 0L;
    long b = 1L;
    float c = 0f;
    float d = 1f;
    float e = 2f;
    double g = 0.0;
    double h = 1.0;
    int big = 100000;
    long bl = 123456789012L;
    float fv = 3.5f;
    double dv = 2.25;
    String s = "hié";
    Class<?> k = String.class;
    Class<?> ka = int[].class;
    Object n = null;
    int bp = 100;
    int sp = 1000;
    int m1 = -1;
    int z = 0;
    int o = 1;
    int five = 5;
    so = s;
    so = k;
    so = ka;
    so = n;
    return a + b + (long) c + (long) d + (long) e + (long) g + (long) h + big + bl + (long) fv
        + (long) dv + bp + sp + m1 + z + o + five;
  }

  static long arrays(int[] ia, long[] la, float[] fa, double[] da, Object[] oa, byte[] ba,
      char[] ca, short[] sa, boolean[] za, int i) {
    ia[i] = ia[i + 1];
    la[i] = la[i + 1];
    fa[i] = fa[i + 1];
    da[i] = da[i + 1];
    oa[i] = oa[i + 1];
    ba[i] = ba[i + 1];
    ca[i] = ca[i + 1];
    sa[i] = sa[i + 1];
    za[i] = za[i + 1];
    ia[i] += 5;
    la[i] += 5L;
    da[i] *= 2.0;
    int x = ia[i]++;
    long y = la[i]++;
    double w = da[i]--;
    int len = ia.length;
    return x + y + (long) w + len;
  }

  long fields(int v, long w, double q) {
    fi = v;
    fl = w;
    fd = q;
    si = v;
    sl = w;
    sd = q;
    fi += v;
    fl += w;
    int a = fi++;
    long b = fl++;
    double c = fd++;
    int d = si++;
    long e = sl++;
    double f = sd--;
    int g = fi = v;
    long h = fl = w;
    double k = fd = q;
    si = fi = v;
    sl = fl = w;
    return a + b + (long) c + d + e + (long) f + g + h + (long) k;
  }

  static int iinc(int i) {
    i += 5;
    i -= 7;
    i++;
    i--;
    i += 200;
    i -= 30000;
    return i;
  }

  static Object objs(Object o) {
    String s = (String) o;
    boolean b = o instanceof String;
    int[][] m = new int[3][4];
    long[][][] m3 = new long[2][3][];
    Object[] arr = new Object[5];
    String[][] sarr = new String[2][];
    boolean[] z = new boolean[1];
    char[] c = new char[2];
    float[] f = new float[3];
    double[] d = new double[4];
    byte[] bt = new byte[5];
    short[] sh = new short[6];
    int[] in = new int[7];
    long[] l = new long[8];
    so = s;
    so = b ? m : m3;
    so = arr;
    so = sarr;
    so = z;
    so = c;
    so = f;
    so = d;
    so = bt;
    so = sh;
    so = in;
    return l;
  }

  static int ternary(int a, Object o) {
    int x = a > 0 ? a : -a;
    Object y = o != null ? o : so;
    so = y;
    return x;
  }

  static void sync(Object o) {
    synchronized (o) {
      si++;
    }
  }

  static int sw(int k) {
    switch (k) {
      case 1: return 10;
      case 2: return 20;
      case 3: return 30;
      default: return 0;
    }
  }

  static int lsw(int k) {
    switch (k) {
      case 1: return 1;
      case 1000: return 2;
      case 100000: return 3;
      default: return 0;
    }
  }

  static void thr(RuntimeException e) {
    throw e;
  }

  static long rl(long a) { return a; }
  static float rf(float a) { return a; }
  static double rd(double a) { return a; }
  static Object ro(Object a) { return a; }
  static void rv() { return; }

  static int trycatch(int[] a) {
    try {
      return a[0];
    } catch (RuntimeException e) {
      so = e;
      return -1;
    }
  }

  static int calls(Object o) {
    int h = o.hashCode();
    String s = String.valueOf(h);
    so = s;
    return h;
  }
}
