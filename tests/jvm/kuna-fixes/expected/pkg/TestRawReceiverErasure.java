package pkg;

import java.util.Iterator;

public class TestRawReceiverErasure {
   private TestRawReceiverErasure.Bag names;

   public String[] all(TestRawReceiverErasure.Bag bag) {
      return (String[])bag.keys().toArray(new String[0]);
   }

   public String[] fields() {
      return (String[])this.names.toArray(new String[0]);
   }

   public int count(TestRawReceiverErasure.Bag bag) {
      int n = 0;

      for (String s : (TestRawReceiverErasure.Bag<String>)bag) {
         n += s.length();
      }

      return n;
   }

   public int total(TestRawReceiverErasure.Holder<Object> holder) {
      int n = 0;

      for (String s : (TestRawReceiverErasure.Bag<String>)holder.take()) {
         n += s.length();
      }

      return n;
   }

   public int first(TestRawReceiverErasure.Bag<String> bag) {
      return bag.get().length() + bag.keys().toArray(new String[0]).length;
   }

   static class Bag<E> implements Iterable<E> {
      E first;

      public Iterator<E> iterator() {
         return null;
      }

      <T> T[] toArray(T[] a) {
         return a;
      }

      TestRawReceiverErasure.Bag<E> keys() {
         return this;
      }

      E get() {
         return this.first;
      }
   }

   static class Holder<V> {
      V value;

      V take() {
         return this.value;
      }
   }
}
