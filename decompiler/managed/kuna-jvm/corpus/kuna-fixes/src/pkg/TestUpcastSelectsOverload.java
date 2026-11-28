// An upcast the printer leaves out still selects the overload: javac -g --release 17
package pkg;

public class TestUpcastSelectsOverload {
   interface Named {
      String name();
   }

   static class Item implements Named {
      public String name() {
         return "item";
      }
   }

   static class Box<T> {
      T value;

      T get() {
         return value;
      }
   }

   static int arr(Named[] n) {
      return 1;
   }

   static int arr(Item[] i) {
      return 2;
   }

   static int take(Named n) {
      return 1;
   }

   static int take(Item i) {
      return 2;
   }

   static int calls(Item[] items, Box<Item> box) {
      return arr((Named[]) items) + arr(items) + take((Named) box.get()) + take(box.get());
   }
}
