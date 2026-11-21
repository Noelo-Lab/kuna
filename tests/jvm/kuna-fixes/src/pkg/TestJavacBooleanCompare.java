package pkg;

public class TestJavacBooleanCompare {
    interface Box {
        boolean isEmpty();

        boolean has(Object o);
    }

    static final boolean DEFAULT_ON = true;

    Box items;
    boolean flag;

    boolean nonEmpty() {
        return false == items.isEmpty();
    }

    boolean guard(Object o) {
        if (false == items.has(o)) {
            return false;
        }
        return true;
    }

    boolean either(Box b) {
        return (b == null || b.isEmpty()) == false;
    }

    boolean both(long a, double b) {
        return (a == 0 && b == 0) == false;
    }

    boolean onTrue(Object o) {
        return items.has(o) == true;
    }

    boolean changed() {
        return flag != DEFAULT_ON;
    }

    boolean plain(Object o) {
        return items.has(o) == false;
    }

    boolean notBox(Object o) {
        if (false == o instanceof Box) {
            return false;
        }
        Box b = (Box) o;
        return b.isEmpty();
    }
}
