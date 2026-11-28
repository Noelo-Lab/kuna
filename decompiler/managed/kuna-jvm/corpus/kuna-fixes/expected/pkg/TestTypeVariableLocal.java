package pkg;

import java.util.Iterator;

public class TestTypeVariableLocal<T extends Number> {
   <U extends T> void fromArray(U[] var1, TestTypeVariableLocal.Sink<T> var2) {
      U var3 = var1[0];
      var2.put(var3);
   }

   static <N extends Number> void forEach(TestTypeVariableLocal.Pairs<String, N> var0, TestTypeVariableLocal.Bag<N> var1) {
      for (N var3 : var1) {
         var0.put(var3.toString(), var3);
      }
   }

   static <C extends Comparable<C>> C max(TestTypeVariableLocal.Bag<C> var0) {
      C var1 = null;

      for (C var3 : var0) {
         var1 = var1 == null || var1.compareTo(var3) < 0 ? var3 : var1;
      }

      return var1;
   }

   static class Bag<E> implements Iterable<E> {
      public Iterator<E> iterator() {
         return null;
      }
   }

   interface Pairs<K, V> {
      void put(K var1, V var2);
   }

   interface Sink<X> {
      void put(X var1);
   }
}
