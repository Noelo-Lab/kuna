package pkg;

public class TestSwitchCaseOrder {
    enum Opt { POINTS, DOC_VALUES }

    static int sink;

    static String style(char c) {
        return switch (c) {
            case 'M' -> "medium";
            case 'L' -> "long";
            case 'F' -> "full";
            case '-' -> null;
            default -> throw new IllegalArgumentException("bad " + c);
        };
    }

    static int query(Opt o, long v) {
        return switch (o) {
            case DOC_VALUES -> (int) v;
            case POINTS -> (int) (v + 1);
        };
    }

    static void arrows(int x) {
        switch (x) {
            case 7 -> sink = 7;
            case 1 -> sink = 1;
            case 4 -> sink = 4;
            default -> throw new IllegalStateException();
        }
        sink++;
    }

    static int fall(int x) {
        int r = 0;
        switch (x) {
            case 9:
                r = 1;
            case 2:
                r += 2;
                break;
            case 5:
                r = 7;
        }
        return r;
    }

    static int middle(int x) {
        switch (x) {
            case 30:
                return 3;
            default:
                return 0;
            case 10:
                return 1;
        }
    }
}
