package pkg;

public class TestReferenceUnboxing {
    interface Sink {
        void add(Object o);
    }

    static Integer field;

    static Object inner(Object v) {
        return v;
    }

    static Long nextDown(Object value) {
        return (long) inner(value);
    }

    static Boolean flag(Object v) {
        return v instanceof Boolean ? (Boolean) v : null;
    }

    static Boolean readFlag(Object value, boolean defaultValue) {
        if (value == null) {
            return defaultValue;
        } else {
            return flag(value).booleanValue();
        }
    }

    static Integer rebox(Integer x) {
        return x.intValue();
    }

    static void store(Integer x) {
        field = x.intValue();
    }

    static void add(Sink sink, Integer x) {
        sink.add(x.intValue());
    }

    static Object[] array(Integer x) {
        return new Object[] {x.intValue()};
    }

    static boolean same(Integer x, Integer y) {
        return x.intValue() == y.intValue();
    }

    static boolean differ(Double x, Double y) {
        return x != null && x.doubleValue() != y.doubleValue();
    }

    static Integer choose(Integer x, boolean c) {
        return c ? x.intValue() : 0;
    }

    static int unboxed(Integer x) {
        return x.intValue() + 1;
    }
}
