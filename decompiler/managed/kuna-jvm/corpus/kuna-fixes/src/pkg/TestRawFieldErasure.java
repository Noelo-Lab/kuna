package pkg;

public class TestRawFieldErasure {
    interface Unit<U extends Unit<U>> extends Comparable<U> {
    }

    abstract static class Base<E extends Comparable<E>> {
    }

    abstract static class Store<E extends Comparable<E>> extends Base<E> {
        E[] data;
    }

    abstract static class UnitStore<N extends Unit<N>> extends Store<N> {
    }

    static <M extends Unit<M>> void fill(M[] target, M[] source) {
    }

    static class Holder<N extends Unit<N>> {
        void copy(UnitStore<N> into, Base<N> from) {
            fill(into.data, ((UnitStore<N>) from).data);
        }
    }
}
