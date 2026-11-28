// Compiled with ECJ 3.46.100 (org.eclipse.jdt:ecj): java -jar ecj-3.46.100.jar -g -17 -d kuna-fixes/classes
package pkg;

public class TestEcjReturnedCondition {
    int count;
    int[] bits;

    boolean empty() {
        return count == 0;
    }

    boolean within(int low, int high) {
        return count >= low && count <= high;
    }

    boolean either(boolean other) {
        return count > 2 || other;
    }

    boolean flagged(int index) {
        if (bits != null)
            return (bits[index] & 4) != 0;
        return false;
    }

    static boolean same(Object a, Object b) {
        if (a == null) {
            return b == null;
        }
        if (b == null)
            return false;
        return a.equals(b);
    }

    boolean explicit() {
        if (count > 3)
            return true;
        return false;
    }
}
