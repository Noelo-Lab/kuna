package pkg;

public class TestConcatOperandType {
    boolean closed = true;
    String low = "1";
    String high = "2";
    float f = 0.1F;
    int n = 66;
    char c = 'A';
    long l = 3L;

    public String toString() {
        return "range" + ":" + (closed ? '[' : '(') + low + ',' + high + (closed ? ']' : ')');
    }

    static String describe(char c2) {
        return "char '" + c2 + "' with value " + (int) c2;
    }

    String widened() {
        return "f=" + (double) f + " n=" + (float) n + " d=" + (double) n + " l=" + (double) l + " L=" + (long) n;
    }

    String chars() {
        return "c=" + (char) n + (char) (n + 1) + " i=" + (int) c + (c + 1) + 'x' + 65;
    }

    String append(String s, char ch, float x) {
        s += (int) ch;
        s += (double) x;
        return s;
    }

    static String values(char ch, float x) {
        return String.valueOf((int) ch) + String.valueOf((double) x);
    }
}
