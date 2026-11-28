package pkg;

public class TestUncheckedCallResultCast {
  static class Shape {
  }

  static class Box<V> {
    V value;

    Box<V> self() {
      return this;
    }
  }

  static class Shelf<B extends Shape> {
    Box<B> items() {
      return new Box<>();
    }

    @SuppressWarnings({"rawtypes", "unchecked"})
    Box<Shape> all() {
      return (Box) items();
    }
  }

  static class Kind<T> {
    Box<?> any() {
      return new Box<>();
    }

    Box<T> exact() {
      return new Box<>();
    }

    <R> Box<R> generic() {
      return new Box<>();
    }
  }

  static class Factory<P> {
    Box<P> make() {
      return new Box<>();
    }
  }

  @SuppressWarnings("unchecked")
  <T> Box<T> resolved(Kind<?> kind) {
    return (Box<T>) kind.any();
  }

  @SuppressWarnings("unchecked")
  <T> T first(Kind<?> kind) {
    Box<T> box = (Box<T>) kind.any();
    return box.value;
  }

  @SuppressWarnings("unchecked")
  Box<Shape> shapes(Factory<? extends Shape> factory) {
    return (Box<Shape>) factory.make();
  }

  @SuppressWarnings("unchecked")
  Box<Shape> chained(Factory<? extends Shape> factory) {
    return (Box<Shape>) factory.make().self();
  }

  Box<String> exact(Kind<String> kind) {
    return kind.exact();
  }

  Box<Shape> generic(Kind<?> kind) {
    return kind.generic();
  }
}
