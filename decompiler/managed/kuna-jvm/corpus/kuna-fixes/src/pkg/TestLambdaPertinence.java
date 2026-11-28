package pkg;

public class TestLambdaPertinence {
  interface Fn<A, R> {
    R apply(A a);
  }

  interface ToInt<T> {
    int applyAsInt(T value);
  }

  interface Task<V> {
    V call();
  }

  interface Action {
    void run();
  }

  interface It<T> {
    T next();
  }

  interface Flow<T> {
    <R> Flow<R> map(Fn<? super T, ? extends R> fn);

    It<T> iterator();
  }

  interface Order<T> {
    int compare(T a, T b);

    default Order<T> reversed() {
      return (a, b) -> compare(b, a);
    }

    static <T> Order<T> comparingInt(ToInt<? super T> key) {
      return (a, b) -> key.applyAsInt(a) - key.applyAsInt(b);
    }
  }

  static class Box<V> {
    V value;

    int size() {
      return 1;
    }
  }

  static class Name {
    String text;

    int length() {
      return text.length();
    }

    void clear() {
      text = "";
    }
  }

  static final Order<Box<?>> BY_SIZE = Order.<Box<?>>comparingInt(Box::size).reversed();

  static <T> It<T> single(T element) {
    return () -> element;
  }

  It<Task<Integer>> tasks(Flow<Name> names) {
    return names.<Task<Integer>>map(n -> () -> n.length()).iterator();
  }

  It<Action> actions(Flow<Name> names) {
    return names.<Action>map(n -> n::clear).iterator();
  }

  Order<Box<String>> byValue() {
    return Order.<Box<String>>comparingInt(b -> b.value.length()).reversed();
  }

  int first() {
    return TestLambdaPertinence.<Task<Integer>>single(() -> 1).next().call();
  }

  Order<Name> byLength() {
    return Order.comparingInt(Name::length).reversed();
  }

  It<Integer> lengths(Flow<Name> names) {
    return names.map(Name::length).iterator();
  }

  It<String> texts(Flow<Name> names) {
    return names.map(n -> n.text).iterator();
  }
}
