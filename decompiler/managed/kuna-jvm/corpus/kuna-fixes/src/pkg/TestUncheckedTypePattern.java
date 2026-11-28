package pkg;

import java.util.List;

public class TestUncheckedTypePattern {
  interface Box<T> {
    T get();
  }

  interface Node {
  }

  static class Leaf<T> implements Box<T>, Node {
    public T get() {
      return null;
    }
  }

  static class Pair<A, B> implements Box<A> {
    public A get() {
      return null;
    }
  }

  @SuppressWarnings("unchecked")
  public int size(Object value) {
    if (value instanceof List) {
      List<String> list = (List<String>) value;
      return list.get(0).length();
    }
    return -1;
  }

  static class Grammar<N extends Leaf<String>> {
    final java.util.Map<String, N> named = new java.util.HashMap<>();

    @SuppressWarnings("unchecked")
    void add(Iterable<Node> nodes) {
      for (Node node : nodes) {
        if (node instanceof Leaf) {
          N leaf = (N) node;
          named.put(leaf.toString(), leaf);
        }
      }
    }
  }

  public String fromBox(Box<String> box) {
    String s = "";
    if (box instanceof Leaf<String> leaf) {
      s = leaf.get();
    }
    return s;
  }

  @SuppressWarnings("unchecked")
  public Integer fromPair(Box<String> box) {
    if (box instanceof Pair) {
      Pair<String, Integer> pair = (Pair<String, Integer>) box;
      return pair.get().length();
    }
    return null;
  }

  public int wildcard(Object value) {
    int n = 0;
    if (value instanceof List<?> list) {
      n = list.size();
    }
    return n;
  }
}
