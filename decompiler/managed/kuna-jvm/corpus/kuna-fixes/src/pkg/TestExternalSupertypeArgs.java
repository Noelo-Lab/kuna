package pkg;

public class TestExternalSupertypeArgs {
    enum IdentityFunction implements ExtFunction<Object, Object> {
        INSTANCE;

        public Object apply(Object o) {
            return o;
        }
    }

    enum NullEntry implements ExtEntry<Object, Object> {
        INSTANCE;

        public Object key() {
            return null;
        }
    }

    static class Holder<V> {
        @SuppressWarnings({"unchecked", "rawtypes"})
        ExtEntry<String, V> entry() {
            ExtEntry<String, V> result = (ExtEntry) NullEntry.INSTANCE;
            return result;
        }
    }

    @SuppressWarnings("unchecked")
    public static <E> ExtFunction<E, E> identity() {
        return (ExtFunction<E, E>) IdentityFunction.INSTANCE;
    }

    @SuppressWarnings("unchecked")
    static <K, V> ExtEntry<K, V> nullEntry() {
        return (ExtEntry<K, V>) (ExtEntry<?, ?>) NullEntry.INSTANCE;
    }

    public static void main(String[] args) {
        ExtFunction<String, String> f = identity();
        System.out.println(f.apply("x") + " " + nullEntry().key() + " " + new Holder<Integer>().entry().key());
    }
}

interface ExtFunction<F, T> {
    T apply(F f);
}

interface ExtEntry<K, V> {
    K key();
}
