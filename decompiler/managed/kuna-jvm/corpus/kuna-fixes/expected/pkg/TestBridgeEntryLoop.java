package pkg;

import java.util.Map.Entry;

public class TestBridgeEntryLoop<V extends Comparable<V>> extends TestBridgeEntryLoopBase<V> {
   static int keyLength(TestBridgeEntryLoop<? extends Comparable<?>> table) {
      int length = 0;

      for (Entry<String, ? extends Comparable<?>> entry : table.entries()) {
         length += ((String)entry.getKey()).length();
      }

      return length;
   }
}
