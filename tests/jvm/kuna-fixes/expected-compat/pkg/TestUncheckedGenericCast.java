package pkg;

public abstract class TestUncheckedGenericCast<K, V> {
   static final TestUncheckedGenericCast.Ordering<Comparable<?>> NATURAL = TestUncheckedGenericCast.Natural.INSTANCE;
   TestUncheckedGenericCast.Ordering<? super K> keyOrder;
   Iterable<TestUncheckedGenericCast.Entry<K, V>> entries;

   static <K, V> TestUncheckedGenericCast.Entry<K, V> nullEntry() {
      return TestUncheckedGenericCast.NullEntry.INSTANCE;
   }

   <K1 extends K, V1 extends V> TestUncheckedGenericCast<K1, V1> retype() {
      TestUncheckedGenericCast<K1, V1> me = this;
      me.keyOrder = null;
      return me;
   }

   static <C extends Comparable<?>> TestUncheckedGenericCast.Ordering<C> natural() {
      return NATURAL;
   }

   TestUncheckedGenericCast.Ordering<Object> unsafeOrder() {
      return this.keyOrder;
   }

   <E extends TestUncheckedGenericCast.Entry<K, V>> void setEntries(Iterable<E> source) {
      this.entries = source;
   }

   TestUncheckedGenericCast.Ordering<? super K> order() {
      return this.keyOrder;
   }

   interface Entry<K, V> {
      K key();
   }

   static final class Natural implements TestUncheckedGenericCast.Ordering<Comparable<?>> {
      static final TestUncheckedGenericCast.Natural INSTANCE = new TestUncheckedGenericCast.Natural();

      public int compare(Comparable<?> a, Comparable<?> b) {
         return 0;
      }
   }

   enum NullEntry implements TestUncheckedGenericCast.Entry<Object, Object> {
      INSTANCE;

      @Override
      public Object key() {
         return null;
      }
   }

   interface Ordering<T> {
      int compare(T var1, T var2);
   }
}
