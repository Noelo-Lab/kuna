package pkg;

public class TestLocalGenericNoDebug {
   private final TestLocalGenericNoDebug.Registry registry = new TestLocalGenericNoDebug.Registry();

   int prune(String var1) {
      TestLocalGenericNoDebug.Bag<String> var2 = this.registry.getNames();
      var2.removeIf(var1x -> var1x.startsWith(var1));
      return var2.size();
   }

   int total() {
      TestLocalGenericNoDebug.Seq<TestLocalGenericNoDebug.Entry> var1 = this.registry.open();
      int var2 = 0;

      for (TestLocalGenericNoDebug.Entry var4 : var1) {
         var2 += var4.size;
      }

      return var2 + var1.size();
   }

   int added(Object var1) {
      TestLocalGenericNoDebug.Bag var2 = this.registry.getNames();
      var2.add(var1);
      return var2.size();
   }

   interface Bag<T> {
      boolean removeIf(TestLocalGenericNoDebug.Test<? super T> var1);

      void add(T var1);

      int size();
   }

   static class Entry {
      String name;
      int size;
   }

   static class Registry {
      TestLocalGenericNoDebug.Bag<String> names;
      TestLocalGenericNoDebug.Seq<TestLocalGenericNoDebug.Entry> entries;

      TestLocalGenericNoDebug.Bag<String> getNames() {
         return this.names;
      }

      TestLocalGenericNoDebug.Seq<TestLocalGenericNoDebug.Entry> open() {
         return this.entries;
      }
   }

   interface Seq<T> extends Iterable<T> {
      int size();
   }

   interface Test<T> {
      boolean test(T var1);
   }
}
