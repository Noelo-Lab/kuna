import java.util.List;
import java.util.function.BiFunction;
import java.util.function.Consumer;
import java.util.function.Function;
import java.util.function.Supplier;
import java.util.function.ToIntFunction;
import java.util.function.ToLongFunction;

public class PropCases {
  private int x;

  public void asserts(int a) {
    assert a > 0;
    assert a < 100 : "too big";
  }

  public void lambdas(String captured) {
    ToIntFunction<String> length = s -> s.length();
    Consumer<StringBuilder> reset = sb -> sb.setLength(0);
    Consumer<String> discard = s -> { s.trim(); };
    ToLongFunction<Long> unbox = l -> l.longValue();
    Function<String, String> concat = s -> s.concat(captured);
    Function<String, Integer> chained = s -> s.trim().length();
    ToIntFunction<List<String>> size = l -> l.size();
    Function<String, String> constArg = s -> s.substring(1);
    Supplier<String> stat = () -> String.valueOf(3);
    Supplier<Integer> instance = () -> this.x + 1;
    BiFunction<String, Integer, Character> charAt = (s, i) -> s.charAt(i);
    Function<String, String> nested = s -> {
      Function<String, String> inner = t -> t.toUpperCase();
      return inner.apply(s);
    };
    Runnable empty = () -> { };
    use(length, reset, discard, unbox, concat, chained, size, constArg, stat, instance, charAt, nested, empty);
  }

  private static void use(Object... values) {
  }

  static class Inner {
    static final int CONST = 7;
    static int counter = 3;

    void check(int v) {
      assert v != CONST;
    }
  }

  interface Shape {
    static void verify(int sides) {
      assert sides >= 3 : sides;
    }
  }

  static class NoAssert {
    static final String NAME;

    static {
      NAME = String.valueOf(System.nanoTime());
    }
  }

  enum Color {
    RED, GREEN;

    void check() {
      assert ordinal() >= 0;
    }
  }

  class Accessors {
    int read() {
      return x;
    }

    void write() {
      x = 4;
    }
  }
}
