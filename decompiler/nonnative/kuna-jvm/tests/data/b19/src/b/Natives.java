package b;

public abstract class Natives {
  int var1;

  public abstract void m(int a, long b, String c);

  public native void n(double d, int e);

  public static native void s(long x, Object y, int z);

  abstract int noArgs();
}
