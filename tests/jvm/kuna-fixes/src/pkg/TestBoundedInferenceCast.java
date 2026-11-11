package pkg;

public class TestBoundedInferenceCast {
    interface Rank<T> {
        int rank(T other);
    }

    interface Cmp<T> {
        int compare(T a, T b);
    }

    static class Ord<T> implements Cmp<T> {
        @SuppressWarnings("rawtypes")
        static <C extends Rank> Ord<C> natural() {
            return new Ord<>();
        }

        public int compare(T a, T b) {
            return 0;
        }

        <S extends T> Ord<S> reverse() {
            return new Ord<>();
        }
    }

    @SuppressWarnings("unchecked")
    static <E> Cmp<E> orNatural(Cmp<E> c) {
        if (c != null) {
            return c;
        }
        return (Cmp<E>) Ord.natural();
    }

    @SuppressWarnings("unchecked")
    static <E> int copy(E a, E b) {
        Ord<E> o = (Ord<E>) Ord.natural();
        return o.compare(a, b);
    }

    @SuppressWarnings("unchecked")
    static <E> Cmp<? super E> reversed() {
        return (Cmp<? super E>) Ord.natural().reverse();
    }

    @SuppressWarnings({"unchecked", "rawtypes"})
    static Cmp<Object> objects() {
        return (Cmp) Ord.natural();
    }

    static <E extends Rank<E>> Cmp<E> ranked() {
        return Ord.natural();
    }

    public static void main(String[] args) {
        System.out.println(orNatural(null).compare("a", "b") + copy(1, 2) + reversed().compare(null, null)
                + objects().compare(1, 2) + (ranked() == null ? 1 : 0));
    }
}
