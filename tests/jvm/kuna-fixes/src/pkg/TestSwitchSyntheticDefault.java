package pkg;

public class TestSwitchSyntheticDefault {
    enum Kind { ONE }

    enum Format { FLAT, TREE }

    sealed interface Shape permits Circle, Square {}

    static final class Circle implements Shape {
        int r;
    }

    static final class Square implements Shape {
        int s;
    }

    static long visit(Kind k, long n) {
        return switch (k) {
            case ONE -> n * 2;
        };
    }

    static long both(Format f, long n) {
        return switch (f) {
            case FLAT -> n * 2;
            case TREE -> throw new IllegalStateException("tree");
        };
    }

    static int area(Shape s) {
        return switch (s) {
            case Circle c -> c.r * c.r * 3;
            case Square q -> q.s * q.s;
        };
    }

    static String name(Shape s) {
        String n = switch (s) {
            case Circle c -> "circle";
            case Square q -> "square";
        };
        return n + "!";
    }

    static int sink;

    static void describe(Shape s) {
        switch (s) {
            case Circle c -> sink = c.r;
            case Square q -> sink = q.s;
        }
    }
}
