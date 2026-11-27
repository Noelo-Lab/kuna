package pkg;

public class TestDiamondWildcardArg {
   interface Pair<A, B> {
   }

   interface Bag<T> {
   }

   static class Holder<K, V> {
      Holder(Pair<? extends K, ? extends Bag<? extends V>> p) {
      }
   }

   static class Wrapper<V> {
      Object[] items;

      @SuppressWarnings("unchecked")
      Object[] wrap() {
         Object[] out = new Object[this.items.length];
         for (int i = 0; i < this.items.length; i++) {
            if (this.items[i] instanceof Pair<?, ?> p) {
               out[i] = new Holder<>((Pair<?, ? extends Bag<? extends V>>)p);
            }
         }
         return out;
      }
   }
}
