package pkg;

public class TestImplicitWideningArithmetic {
    static long negatedProduct(int size, int count) {
        return -(long) size * count;
    }

    static long negated(int a) {
        return -(long) a;
    }

    static double negatedDouble(int a) {
        return -(double) a;
    }

    static long remainderProduct(int a, int b, int c) {
        return ((long) a % b) * c;
    }

    static long maskedShift(int a, int b, int n) {
        return ((long) a & b) << n;
    }

    static long choiceProduct(boolean c, int a, int b, int d) {
        return (c ? (long) a : b) * d;
    }

    static double remainder(int a, int b) {
        return (double) a % (double) b;
    }

    static long product(int a, int b) {
        return (long) a * b;
    }

    static long shifted(int a, int n) {
        return (long) a << n;
    }

    static long widenedResult(int a, int b) {
        long r = a * b;
        return r - a;
    }
}
