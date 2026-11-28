package pkg;

public class TestVariableArrayErasure {
   static class Item {
   }

   static class Seq<X> {
      X[] items() {
         return null;
      }
   }

   static <K extends Item> int count(Seq<K> s) {
      int n = 0;
      for (K k : s.items()) {
         n += k.hashCode();
      }
      K[] all = s.items();
      Item[] items = s.items();
      return n + all.length + items.length;
   }
}
