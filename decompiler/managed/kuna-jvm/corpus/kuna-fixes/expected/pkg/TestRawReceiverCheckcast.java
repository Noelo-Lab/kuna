package pkg;

public class TestRawReceiverCheckcast<E extends CharSequence> {
   E item;

   public void split(TestRawReceiverCheckcast.Page<String, Integer> var1, E var2) {
      TestRawReceiverCheckcast.Bag var3 = new TestRawReceiverCheckcast.Bag();
      var3.add((String)var1.getKey(0));
      var3.add((String)var1.key);
      var3.add(var1.name());
      var3.addBoth((String)var1.getKey(1), var1.getKey(2));
      TestRawReceiverCheckcast.Bag var4 = new TestRawReceiverCheckcast.Bag();
      var4.add((Integer)var1.value);
      var4.add((Integer)var1.pick(3));
      TestRawReceiverCheckcast.Bag var5 = new TestRawReceiverCheckcast.Bag();
      var5.add((CharSequence)var1.getKey(3));
      var5.add(var2);
      var5.add(this.item);
   }

   public void typed(TestRawReceiverCheckcast.Page<String, Integer> var1, TestRawReceiverCheckcast.Bag<String> var2) {
      var2.add((String)var1.getKey(0));
      var2.add(var1.key);
   }

   static class Bag<T> {
      void add(T var1) {
      }

      void addBoth(T var1, Object var2) {
      }
   }

   static class Page<K, V> {
      K key;
      V value;

      K getKey(int var1) {
         return this.key;
      }

      String name() {
         return "p";
      }

      <X> X pick(X var1) {
         return (X)var1;
      }
   }
}
