package pkg;

public class TestGenericPatternBinding {
    interface Source<T> {
        T get();
    }

    static final class Box<V> implements Source<Box<V>> {
        public Box<V> get() {
            return this;
        }
    }

    static <V> Box<V> unwrap(Source<Box<V>> source) {
        return source instanceof Box<V> box ? box : source.get();
    }

    static <V> Box<V> open(Source<Box<V>> source) {
        if (source instanceof Box<V> box) {
            return box;
        }
        return source.get();
    }
}
