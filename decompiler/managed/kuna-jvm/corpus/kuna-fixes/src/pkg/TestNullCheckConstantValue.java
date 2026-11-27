package pkg;

import java.util.Objects;

public class TestNullCheckConstantValue {
    static class Box {
        final int k = 3;
        final String name = "box";
    }

    class Sub {
        Sub(Box b) {
        }

        int v() {
            return 1;
        }
    }

    static int foo(int a, Sub s) {
        return a + s.v();
    }

    int anonymous(Box x) {
        Objects.requireNonNull(x);
        return foo(5, new Sub(x) {});
    }

    static int other(Box b) {
        Objects.requireNonNull(b);
        return 7;
    }

    static int constant(Box b) {
        return b.k;
    }

    static String name(Box b) {
        return b.name;
    }

    static int string(String s) {
        Objects.requireNonNull(s);
        return 5;
    }

    static int object(Object o) {
        o.getClass();
        return 9;
    }
}
