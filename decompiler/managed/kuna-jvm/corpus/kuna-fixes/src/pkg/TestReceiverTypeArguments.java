package pkg;

public class TestReceiverTypeArguments {
    interface Iter<T> {
        T next();
    }

    static class Lst<E> {
        final E first;

        Lst(E first) {
            this.first = first;
        }

        static <E> Lst<E> of(E e) {
            return new Lst<>(e);
        }

        static <E> Lst<E> of(E a, E b) {
            return new Lst<>(a);
        }

        Iter<E> iterator() {
            return new Iter<E>() {
                public E next() {
                    return first;
                }
            };
        }
    }

    static class Setting<T> {
    }

    static final Setting<Integer> MODE = new Setting<>();
    static final Setting<String> NAME = new Setting<>();

    interface Cmp<T> {
        int compare(T a, T b);
    }

    static class Sorted<E> {
    }

    static class Builder<E> {
        final Cmp<? super E> cmp;

        Builder(Cmp<? super E> cmp) {
            this.cmp = cmp;
        }

        Builder<E> add(E e) {
            return this;
        }

        Sorted<E> build() {
            return new Sorted<>();
        }
    }

    static Iter<Setting<?>> settings() {
        return Lst.<Setting<?>>of(MODE).iterator();
    }

    static Iter<Setting<?>> both() {
        return Lst.<Setting<?>>of(MODE, NAME).iterator();
    }

    static <E> Sorted<E> copyOf(Cmp<? super E> cmp, E e) {
        Sorted<E> sorted = new Builder<E>(cmp).add(e).build();
        return sorted;
    }

    static Iter<Setting<Integer>> plain() {
        return Lst.of(MODE).iterator();
    }

    public static void main(String[] args) {
        System.out.println(settings().next() == MODE && both().next() == MODE && plain().next() == MODE
                && copyOf((a, b) -> 0, "x") != null);
    }
}
