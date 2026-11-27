package pkg;

public class TestInnerMethodArgumentShift<K, V> {
  interface Nav<K> {
    Nav<K> sub(K from, boolean fromInc, K to, boolean toInc);

    Nav<K> sub(K from, K to);
  }

  interface Box<T> {
    T get();
  }

  class Inner implements Nav<K> {
    public Nav<K> sub(K from, K to) {
      return sub(from, true, to, false);
    }

    public Nav<K> sub(K from, boolean fromInc, K to, boolean toInc) {
      return this;
    }

    boolean accepts(K key) {
      return key != null;
    }

    @SuppressWarnings("unchecked")
    boolean contains(Object o) {
      Box<?> box = (Box<?>) o;
      return accepts((K) box.get());
    }
  }

  class Segment {
    V value;

    V put(K key, V value, long now) {
      return value;
    }

    boolean remove(K key, V value, boolean force) {
      return force && key != null;
    }
  }

  V store(Segment segment, K key, V value) {
    segment.remove(key, value, true);
    return segment.put(key, value, 0L);
  }
}
