package pkg;

public class TestQualifiedInheritedInner extends TestQualifiedInheritedInnerBase {
    class Values {
        Object iterator() {
            return TestQualifiedInheritedInner.this.new Cursor(false);
        }
    }
}

class TestQualifiedInheritedInnerBase {
    class Cursor {
        final boolean keys;

        Cursor(boolean keys) {
            this.keys = keys;
        }
    }
}
