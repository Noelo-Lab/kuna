import java.io.IOException;
import java.util.*;
import java.util.function.*;

// Compiled by GenF2Golden; its constant pool is a ConstantPool golden vector.
public class PoolSample<T extends Comparable<? super T>> implements Iterable<T> {
  static final long BIG = Long.MIN_VALUE;
  static final double D = -0.0;
  static final float F = Float.NaN;
  static final int I = 123456789;
  static final String S = "été 😀 \ud800 \u0000 end";

  record Point(int x, int y) {}

  enum Color { RED, GREEN }

  interface Shape { double area(); }

  private final List<T> items = new ArrayList<>();
  private Map<String, List<? extends Number>> index;

  public <X extends Throwable> T first(Supplier<? extends X> err) throws X, IOException {
    if (items.isEmpty()) {
      throw err.get();
    }
    return items.get(0);
  }

  public String describe(Object o, int n) {
    String kind = switch (o) {
      case Point p when p.x() > 0 -> "point " + p.x() + "," + p.y();
      case String s -> "string " + s.length();
      case Color c -> "color " + c.ordinal();
      default -> "other" + n;
    };
    Function<String, Integer> len = String::length;
    BiFunction<Integer, Integer, Integer> add = Integer::sum;
    Runnable r = () -> System.out.println(kind + BIG + D + F + I);
    r.run();
    int[][] grid = new int[n][n];
    Object[] arr = new String[1];
    return kind + len.apply(kind) + add.apply(1, 2) + grid.length + arr.getClass() + int[].class;
  }

  @Override
  public Iterator<T> iterator() {
    return items.iterator();
  }

  class Inner<U> {
    U value;
    T outer;
  }
}
