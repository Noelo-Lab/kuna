public abstract class Natives {
  public native int nat(long x);
  protected abstract void abs() throws java.io.IOException, InterruptedException;
  public strictfp double fp(double a) { return a * 2; }
  @SuppressWarnings("deprecation")
  @Deprecated
  public final synchronized void legacy() { }
  public static void main(String... args) { }
}
