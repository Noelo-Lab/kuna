package pkg;

public class TestDiamondWildcardArg {
   interface Bag<T> {
   }

   static class Holder<K, V> {
      Holder(TestDiamondWildcardArg.Pair<? extends K, ? extends TestDiamondWildcardArg.Bag<? extends V>> p) {
      }
   }

   interface Pair<A, B> {
   }

   static class Wrapper<V> {
      Object[] items;

      Object[] wrap() {
         Object[] out = new Object[this.items.length];

         for (int i = 0; i < this.items.length; i++) {
            if (this.items[i] instanceof TestDiamondWildcardArg.Pair<?, ?> p) {
               out[i] = new TestDiamondWildcardArg.Holder<>((TestDiamondWildcardArg.Pair<?, ? extends TestDiamondWildcardArg.Bag<? extends V>>)p);
            }
         }

         return out;
      }
   }
}
