package pkg;

import java.util.function.Supplier;

public class TestImplicitOuterInstance<T> {
    T value;

    class Entry {
        T get() {
            return value;
        }
    }

    class View {
        Entry make() {
            return new Entry();
        }

        Entry qualified() {
            return TestImplicitOuterInstance.this.new Entry();
        }

        Supplier<Entry> later() {
            return () -> new Entry();
        }

        Object anonymous() {
            return new Object() {
                Entry entry() {
                    return new Entry();
                }
            };
        }

        class Deeper {
            Entry make() {
                return new Entry();
            }

            View view() {
                return new View();
            }
        }
    }

    class Sub extends TestImplicitOuterInstance<T> {
        Entry outer() {
            return TestImplicitOuterInstance.this.new Entry();
        }

        Entry own() {
            return new Entry();
        }
    }

    Entry direct() {
        return new Entry();
    }

    static <T> TestImplicitOuterInstance<T>.Entry of(TestImplicitOuterInstance<T> t) {
        return t.new Entry();
    }
}
