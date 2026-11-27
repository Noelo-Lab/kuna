package pkg;

public class TestCompoundAssignmentShape {
    long total;
    int count;
    byte bits;
    char letter;
    int[] slots = new int[4];
    TestCompoundAssignmentShape next;

    static int bump(int[] a) {
        a[0] = 10;
        return 1;
    }

    void addLength(String s) {
        total += s.length();
    }

    void addOther(TestCompoundAssignmentShape other) {
        total += other.total;
    }

    void addPlain(int n) {
        count = count + n;
    }

    void mask(int m) {
        bits &= m;
    }

    void addLong(long delta) {
        count += delta;
    }

    void shiftLetter(int n) {
        letter += n;
    }

    void clearBits(byte[] dest, byte[] src, int i) {
        dest[i] &= ~src[i];
    }

    void addSlot(int i) {
        slots[i] += 2;
    }

    void addNext() {
        next.count = next.count + 3;
    }

    void orFirst(int y) {
        count = y | count;
    }

    static int bumpFirst(int[] a) {
        a[0] = bump(a) + a[0];
        return a[0];
    }
}
