package k7;

import java.util.List;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

public class Fix extends Base implements Iface, Runnable {
  public static final int I = 42;
  public static final long L = 7L;
  static final float F = 1.5f;
  static final double D = 2.25;
  public static final String S = "a\"b\né";
  static final char C = 'x';
  static final boolean Z = true;
  static final int NOT_CONST = Integer.parseInt("1");
  transient int t;
  volatile long v;
  private int p;
  protected String q;
  public List<String> list;
  @TU String tu;
  @TU("w") List<@TU String> tuList;
  /** @deprecated */
  int dep;
  public final Object o = null;
  public static Object so;
  int[][] arr;

  static {
    System.out.println();
  }

  public Fix() {
  }

  Fix(int x) {
  }

  public void run() {
  }

  public void abs() {
  }

  public void ifaceM() {
  }

  protected void prot() {
  }

  public void run0() {
  }

  void pkg() {
  }

  public synchronized void sync() {
  }

  public void thr() throws java.io.IOException, InterruptedException {
  }

  public static void s() {
  }

  private void priv() {
  }

  public final void fm() {
  }

  /** @deprecated */
  public void depM() {
  }

  void withTry() {
    try {
      s();
    } catch (RuntimeException e) {
      priv();
    } finally {
      fm();
    }
  }

  void p1(@NotNull String a, @Nullable String b, @Tag("x") int c, @TU String d, @Both String e,
          @Tag(value = "y", n = 3) @Nullable String f, @TU("z") @Both List<@TU String> g) {
  }

  @Tag
  String visibleRet() {
    return null;
  }

  @Nullable
  String nullableRet() {
    return null;
  }

  @Tag
  @Nullable
  String bugNullable() {
    return null;
  }

  @TU String tuRet() {
    return null;
  }

  class Inner {
    void innerM() {
    }
  }
}
