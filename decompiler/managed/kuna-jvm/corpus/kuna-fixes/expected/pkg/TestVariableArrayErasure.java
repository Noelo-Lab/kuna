package pkg;

public class TestVariableArrayErasure {
   static <K extends TestVariableArrayErasure.Item> int count(TestVariableArrayErasure.Seq<K> s) {
      int n = 0;

      for (K k : s.items()) {
         n += k.hashCode();
      }

      K[] all = (K[])s.items();
      TestVariableArrayErasure.Item[] items = s.items();
      return n + all.length + items.length;
   }

   static class Item {
   }

   static class Seq<X> {
      X[] items() {
         return null;
      }
   }
}
