package w;

import java.util.function.IntSupplier;

public class Dump {
  static int F;
  static String S;
  static long L;
  static double D;
  static float G;
  static Object O;

  static {
    int i = F;
    i += 1000;
    switch (i) {
      case 1: F = 10; break;
      case 2: F = 20; break;
      case 3: F = 30; break;
      default: F = -1;
    }
    switch (i) {
      case 10: F = 1; break;
      case 1000: F = 2; break;
      case -50000: F = 3; break;
    }
    S = "quote\" back\\ nl\n tab\t unié ctl\u0001 cjk中";
    L = 1234567890123L;
    D = 3.5e-10;
    G = 2.5f;
    O = new int[3][4];
    Object[] arr = new String[2];
    long[] la = new long[5];
    if (O instanceof int[][]) {
      O = (int[][]) O;
    }
    O = String.class;
    IntSupplier sup = () -> F + 1;
    F = sup.getAsInt();
    S = "x" + F + "y" + L;
    try {
      F = Integer.parseInt(S);
    } catch (NumberFormatException e) {
      F = -2;
    } finally {
      L++;
    }
    synchronized (Dump.class) {
      F++;
    }
    Runnable r = Dump::new;
    O = r;
    for (int k = 0; k < 300; k++) {
      la[k % 5] += k;
    }
    D = Math.max(D, 1.0) * -0.0;
    G = Float.NaN;
    D = Double.POSITIVE_INFINITY;
    L = Long.MIN_VALUE;
    F = Integer.MIN_VALUE;
    S = null;
    char c = '中';
    F += c + arr.length;
  }

  public Dump() {
    super();
  }

  int typed(int a) {
    return a;
  }

  static int sum(int... xs) {
    return xs.length;
  }

  native long nat(final int a, double b);

  synchronized boolean inst(long a, final char b, int[] c) {
    return a == b;
  }

  static void noParams() {
  }

  private static float[][] arr(byte b, short s) {
    return null;
  }

  protected static final void fin(boolean... flags) {
  }
}
