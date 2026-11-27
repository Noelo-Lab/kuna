package pkg;

import java.util.Iterator;

public class TestTypeVariableLocal<T extends Number> {
   interface Sink<X> {
      void put(X x);
   }

   interface Pairs<K, V> {
      void put(K k, V v);
   }

   static class Bag<E> implements Iterable<E> {
      public Iterator<E> iterator() {
         return null;
      }
   }

   <U extends T> void fromArray(U[] arr, Sink<T> sink) {
      T t = arr[0];
      sink.put(t);
   }

   static <N extends Number> void forEach(Pairs<String, N> map, Bag<N> source) {
      for (N item : source) {
         map.put(item.toString(), item);
      }
   }

   static <C extends Comparable<C>> C max(Bag<C> items) {
      C best = null;
      for (C c : items) {
         best = best == null || best.compareTo(c) < 0 ? c : best;
      }
      return best;
   }
}
