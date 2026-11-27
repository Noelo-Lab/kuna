package pkg;

public class TestInterfaceObjectMethods {
    interface Snapshot extends Cloneable {
        Object clone();

        int size();
    }

    static final class Pair implements Snapshot {
        private final int first;
        private final int second;

        Pair(int first, int second) {
            this.first = first;
            this.second = second;
        }

        public Object clone() {
            return new Pair(first, second);
        }

        public int size() {
            return 2;
        }
    }
}
