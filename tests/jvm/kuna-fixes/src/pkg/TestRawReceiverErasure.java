package pkg;

import java.util.Iterator;

public class TestRawReceiverErasure {
  static class Bag<E> implements Iterable<E> {
    E first;

    public Iterator<E> iterator() {
      return null;
    }

    <T> T[] toArray(T[] a) {
      return a;
    }

    Bag<E> keys() {
      return this;
    }

    E get() {
      return first;
    }
  }

  static class Holder<V> {
    V value;

    V take() {
      return value;
    }
  }

  private Bag names;

  @SuppressWarnings("unchecked")
  public String[] all(Bag bag) {
    return (String[]) bag.keys().toArray(new String[0]);
  }

  @SuppressWarnings("unchecked")
  public String[] fields() {
    return (String[]) this.names.toArray(new String[0]);
  }

  public int count(Bag bag) {
    int n = 0;
    for (Iterator it = bag.iterator(); it.hasNext(); ) {
      String s = (String) it.next();
      n += s.length();
    }
    return n;
  }

  public int total(Holder<Object> holder) {
    int n = 0;
    for (String s : (Bag<String>) holder.take()) {
      n += s.length();
    }
    return n;
  }

  public int first(Bag<String> bag) {
    return bag.get().length() + bag.keys().toArray(new String[0]).length;
  }
}
