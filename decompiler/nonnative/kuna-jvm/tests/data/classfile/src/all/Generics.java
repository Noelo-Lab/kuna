import java.util.*;
import java.util.function.*;

public class Generics<K extends Comparable<? super K>, V> extends AbstractMap<K, V> implements Cloneable {
  private final Map<K, List<? extends V>> map = new HashMap<>();
  public List<? super Integer>[] wild;
  Map.Entry<K, V> entry;

  public Set<Map.Entry<K, V>> entrySet() { return Collections.emptySet(); }

  public static <T extends Number & Comparable<T>> T max(List<? extends T> xs) throws IllegalStateException {
    T best = null;
    for (T x : xs) { if (best == null || x.compareTo(best) > 0) best = x; }
    if (best == null) throw new IllegalStateException();
    return best;
  }

  <R> R apply(Function<? super K, ? extends R> f, K k) {
    List<R> out = new ArrayList<>();
    out.add(f.apply(k));
    return out.get(0);
  }

  class Node<N> { N value; Generics<K, V> owner() { return Generics.this; } }

  static class Pair<A, B> { A a; B b; Pair(A a, B b) { this.a = a; this.b = b; } }

  Node<String> node() { return new Node<>(); }
}
