package pkg;

public class TestInnerMethodArgumentShift<K, V> {
   V store(TestInnerMethodArgumentShift<K, V>.Segment segment, K key, V value) {
      segment.remove(key, value, true);
      return (V)segment.put(key, value, 0L);
   }

   interface Box<T> {
      T get();
   }

   class Inner implements TestInnerMethodArgumentShift.Nav<K> {
      @Override
      public TestInnerMethodArgumentShift.Nav<K> sub(K from, K to) {
         return this.sub(from, true, to, false);
      }

      @Override
      public TestInnerMethodArgumentShift.Nav<K> sub(K from, boolean fromInc, K to, boolean toInc) {
         return this;
      }

      boolean accepts(K key) {
         return key != null;
      }

      boolean contains(Object o) {
         TestInnerMethodArgumentShift.Box<?> box = (TestInnerMethodArgumentShift.Box<?>)o;
         return this.accepts((K)box.get());
      }
   }

   interface Nav<K> {
      TestInnerMethodArgumentShift.Nav<K> sub(K var1, boolean var2, K var3, boolean var4);

      TestInnerMethodArgumentShift.Nav<K> sub(K var1, K var2);
   }

   class Segment {
      V value;

      V put(K key, V value, long now) {
         return value;
      }

      boolean remove(K key, V value, boolean force) {
         return force && key != null;
      }
   }
}
