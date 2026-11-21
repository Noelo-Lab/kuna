package pkg;

public abstract class TestStaticTypeVariableScope<V> {
  interface Box<V> {
    V get();
  }

  interface ListenableBox<V> extends Box<V> {
  }

  static <V> V unwrap(Box<V> box) {
    return box.get();
  }

  static Object value(ListenableBox<?> box) {
    Object v = unwrap(box);
    if (v == null) {
      v = box;
    }
    return v;
  }

  V own(ListenableBox<V> box) {
    return unwrap(box);
  }
}
