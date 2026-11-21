import java.io.Serializable;
import java.util.*;
import java.util.function.*;

public class Lambdas {
  int base = 3;

  Runnable simple() { return () -> System.out.println("hi"); }
  IntUnaryOperator capture(int k) { return x -> x * k + base; }
  Function<String, Integer> ref() { return String::length; }
  Supplier<List<String>> ctor() { return ArrayList::new; }
  IntFunction<int[]> arrayCtor() { return int[]::new; }
  Function<Integer, String> bound(String s) { return s::substring; }
  BiFunction<String, String, Boolean> unbound() { return String::equals; }
  Runnable serial() { return (Runnable & Serializable) () -> base++; }
  String concat(String a, int b, long c, Object d) { return a + b + c + d + "!"; }
  static <T> Comparator<T> cmp(Function<T, String> f) { return Comparator.comparing(f).thenComparing(Object::hashCode); }

  Object nested() {
    return (Supplier<Runnable>) () -> () -> base--;
  }
}
