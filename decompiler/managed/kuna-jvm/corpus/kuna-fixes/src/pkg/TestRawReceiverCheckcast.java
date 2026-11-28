package pkg;

public class TestRawReceiverCheckcast<E extends CharSequence> {
   static class Page<K, V> {
      K key;
      V value;

      K getKey(int i) {
         return this.key;
      }

      String name() {
         return "p";
      }

      <X> X pick(X x) {
         return x;
      }
   }

   static class Bag<T> {
      void add(T t) {
      }

      void addBoth(T a, Object b) {
      }
   }

   E item;

   public void split(Page<String, Integer> p, E e) {
      Bag<String> keys = new Bag<>();
      keys.add(p.getKey(0));
      keys.add(p.key);
      keys.add(p.name());
      keys.addBoth(p.getKey(1), p.getKey(2));
      Bag<Integer> values = new Bag<>();
      values.add(p.value);
      values.add(p.pick(3));
      Bag<CharSequence> chars = new Bag<>();
      chars.add(p.getKey(3));
      chars.add(e);
      chars.add(this.item);
   }

   public void typed(Page<String, Integer> p, Bag<String> out) {
      out.add(p.getKey(0));
      out.add(p.key);
   }
}
