package pkg;

import java.util.ArrayList;
import java.util.Iterator;
import java.util.List;
import java.util.Map;

abstract class TestBridgeEntryLoopBase<V> {
    interface Bag<E> extends Iterable<E> {
    }

    private final List<Map.Entry<String, V>> values = new ArrayList<>();

    public Bag<Map.Entry<String, V>> entries() {
        return new Bag<Map.Entry<String, V>>() {
            public Iterator<Map.Entry<String, V>> iterator() {
                return values.iterator();
            }
        };
    }
}

public class TestBridgeEntryLoop<V extends Comparable<V>> extends TestBridgeEntryLoopBase<V> {
    static int keyLength(TestBridgeEntryLoop<? extends Comparable<?>> table) {
        int length = 0;
        for (Map.Entry<String, ? extends Comparable<?>> entry : table.entries()) {
            length += entry.getKey().length();
        }
        return length;
    }
}
