package b;

import java.util.function.Function;
import java.util.function.Supplier;

public class Outer {
  Runnable nestedLambda = () -> {
    Runnable inner = () -> System.out.println("inner");
    inner.run();
  };
  Supplier<String> reference = this::toString;
  Function<String, Integer> length = String::length;

  public void run() {
    class Local {}
    new Local();
    interface LocalI {}
    enum LocalE { X }
    Object plain = new Object() {};
    Runnable anon = new Runnable() {
      public void run() {
        Runnable l = () -> {};
        l.run();
      }
    };
  }

  public class A {
    Runnable anonymous = new Runnable() {
      public void run() {}
    };

    class Deep {}
  }

  public static class B {}

  private interface C {}
}
