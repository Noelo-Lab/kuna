package pkg;

import java.util.Iterator;

public class TestLocalGenericNoDebug {
   interface Test<T> {
      boolean test(T t);
   }

   interface Bag<T> {
      boolean removeIf(Test<? super T> test);

      void add(T t);

      int size();
   }

   interface Seq<T> extends Iterable<T> {
      int size();
   }

   static class Entry {
      String name;
      int size;
   }

   static class Registry {
      Bag<String> names;
      Seq<Entry> entries;

      Bag<String> getNames() {
         return this.names;
      }

      Seq<Entry> open() {
         return this.entries;
      }
   }

   private final Registry registry = new Registry();

   int prune(String prefix) {
      Bag<String> keys = this.registry.getNames();
      keys.removeIf(k -> k.startsWith(prefix));
      return keys.size();
   }

   int total() {
      Seq<Entry> open = this.registry.open();
      int sum = 0;

      for (Entry e : open) {
         sum += e.size;
      }

      return sum + open.size();
   }

   int added(Object extra) {
      Bag raw = this.registry.getNames();
      raw.add(extra);
      return raw.size();
   }
}
