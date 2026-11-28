package pkg;

public class TestDoubleConstantLiteral {
    static double toDouble(long state) {
        return ((double) ((int) (state >>> 33) + 1)) / 0x1.0p31;
    }

    static double gbFrac(long bytes) {
        return ((double) bytes) / 1073741824.0;
    }

    static boolean below(long bytes) {
        return bytes < 1099511627776.0;
    }

    static Object boxed() {
        return 1073741824.0;
    }

    static String text(StringBuilder sb) {
        return sb.append(1125899906842624.0).toString();
    }

    static double scaled(double d) {
        return d * 1073741824.0;
    }

    static double max(double d) {
        return d * Float.MAX_VALUE;
    }

    static double half() {
        return 0.5;
    }
}
