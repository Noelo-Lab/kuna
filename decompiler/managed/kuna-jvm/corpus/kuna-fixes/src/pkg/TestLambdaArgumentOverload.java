package pkg;

public class TestLambdaArgumentOverload {
  interface IntSink {
    void accept(int value);
  }

  interface Sink<T> {
    void accept(T value);
  }

  interface Lookup<T> {
    T inspect(String name);
  }

  interface Filter {
    boolean matches(String name);
  }

  interface Source {
    void forEach(IntSink sink);

    void forEach(Sink<? super Integer> sink);
  }

  static <T> int select(Class<?> type, Lookup<T> lookup) {
    return 0;
  }

  static int select(Class<?> type, Filter filter) {
    return 1;
  }

  private int total;

  void add(int value) {
    total += value;
  }

  void add(int[] values) {
    total += values.length;
  }

  public void drain(Source source) {
    source.forEach((IntSink) this::add);
    source.forEach((IntSink) v -> total -= v);
  }

  public int lookup() {
    return select(String.class, (Lookup<Boolean>) name -> name.isEmpty() ? Boolean.TRUE : null);
  }

  public int filter() {
    return select(String.class, (Filter) name -> name.isEmpty());
  }

  public int single(Lookup<String> lookup) {
    return lookup.inspect("x").length();
  }
}
