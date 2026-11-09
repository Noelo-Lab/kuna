package pkg;

public abstract class TestUncheckedGenericCast<K, V> {
  interface Entry<K, V> {
    K key();
  }

  interface Ordering<T> {
    int compare(T a, T b);
  }

  enum NullEntry implements Entry<Object, Object> {
    INSTANCE;

    public Object key() {
      return null;
    }
  }

  static final class Natural implements Ordering<Comparable<?>> {
    static final Natural INSTANCE = new Natural();

    public int compare(Comparable<?> a, Comparable<?> b) {
      return 0;
    }
  }

  static final Ordering<Comparable<?>> NATURAL = Natural.INSTANCE;

  Ordering<? super K> keyOrder;
  Iterable<Entry<K, V>> entries;

  @SuppressWarnings("unchecked")
  static <K, V> Entry<K, V> nullEntry() {
    return (Entry<K, V>) (Entry<?, ?>) NullEntry.INSTANCE;
  }

  @SuppressWarnings("unchecked")
  <K1 extends K, V1 extends V> TestUncheckedGenericCast<K1, V1> retype() {
    TestUncheckedGenericCast<K1, V1> me = (TestUncheckedGenericCast<K1, V1>) this;
    me.keyOrder = null;
    return me;
  }

  @SuppressWarnings("unchecked")
  static <C extends Comparable<?>> Ordering<C> natural() {
    return (Ordering<C>) NATURAL;
  }

  @SuppressWarnings("unchecked")
  Ordering<Object> unsafeOrder() {
    return (Ordering<Object>) keyOrder;
  }

  @SuppressWarnings({"unchecked", "rawtypes"})
  <E extends Entry<K, V>> void setEntries(Iterable<E> source) {
    this.entries = (Iterable) source;
  }

  Ordering<? super K> order() {
    return keyOrder;
  }
}
