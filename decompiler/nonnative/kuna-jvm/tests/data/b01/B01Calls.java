// Fixture for tests/b01_invocation.rs: one of each invoke shape javac emits.
import java.util.ArrayList;
import java.util.List;
import java.util.function.Function;
import java.util.function.IntSupplier;
import java.util.function.Supplier;

public class B01Calls extends ArrayList<String> implements Runnable {
  private int f;

  public B01Calls() {
    super();
  }

  public B01Calls(int x) {
    this();
    f = x;
  }

  public void run() {}

  private int priv(int a, long b) {
    return a;
  }

  static String stat(Object o, double d, boolean z) {
    return null;
  }

  int calls(List<String> l, String s) {
    int a = priv(1, 2L);
    stat(s, 1.0, true);
    l.add(s);
    s.length();
    super.size();
    Runnable r = () -> {};
    Supplier<String> sp = s::trim;
    Function<String, Integer> fn = String::length;
    IntSupplier is = () -> f;
    String c = s + a;
    new B01Calls(3);
    Integer.valueOf(a).intValue();
    return a;
  }
}
