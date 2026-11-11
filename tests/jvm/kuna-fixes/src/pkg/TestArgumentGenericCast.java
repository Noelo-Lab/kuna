package pkg;

public class TestArgumentGenericCast {
    interface Bag<K, V> {
        int size();
    }

    static class Props implements Bag<Object, Object> {
        public int size() {
            return 1;
        }
    }

    static class Src<T> {
        final T source;

        Src(String name, T source) {
            this.source = source;
        }
    }

    static class BagSource extends Src<Bag<String, Object>> {
        @SuppressWarnings({"unchecked", "rawtypes"})
        BagSource(String name, Props source) {
            super(name, (Bag) source);
        }
    }

    static class Lst<E> {
        int n;

        boolean add(E e) {
            n++;
            return true;
        }
    }

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
    }

    static class Sorted<E> {
        final Cmp<? super E> cmp;

        @SuppressWarnings("unchecked")
        Sorted() {
            this((Cmp<? super E>) Ord.natural());
        }

        Sorted(Cmp<? super E> cmp) {
            this.cmp = cmp;
        }
    }

    static int writeGeneric(Bag<String, Object> m) {
        return m.size();
    }

    @SuppressWarnings({"unchecked", "rawtypes"})
    static int writeTo(Bag<String, String> inner) {
        return writeGeneric((Bag<String, Object>) (Bag) inner);
    }

    @SuppressWarnings({"unchecked", "rawtypes"})
    static boolean load(Lst<Bag<String, ?>> result, Props props) {
        return result.add((Bag) props);
    }

    public static void main(String[] args) {
        Props p = new Props();
        System.out.println(new BagSource("x", p).source.size() + writeTo(null == args ? null : new Bag<String, String>() {
            public int size() {
                return 2;
            }
        }) + (load(new Lst<>(), p) ? 1 : 0) + (new Sorted<String>().cmp == null ? 0 : 1));
    }
}
