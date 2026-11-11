package p;

import java.util.function.Supplier;

public class Outer {
  public int field;
  Runnable lambda = () -> {};
  Supplier<String> reference = this::toString;

  public void run() {
    class Local {}
    new Local();
  }

  public class A {
    Runnable anonymous = new Runnable() {
      public void run() {}
    };
  }

  public static class B {}
}
