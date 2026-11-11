package pkg;

public abstract class TestDiamondArgumentConflict<K0, W0> {
   interface Supplier<T> {
      T get();
   }

   interface Box<T> {
   }

   abstract static class Base<B extends Base<B>> {
   }

   static final class Key<T> {
   }

   static final class BaseSupplier<V extends Base<V>> implements Supplier<Box<V>> {
      final Key<V> key;

      BaseSupplier(Key<V> key) {
         this.key = key;
      }

      public Box<V> get() {
         return null;
      }
   }

   static <T> T wrap(T t) {
      return t;
   }

   abstract <K extends K0, V extends W0> Supplier<Box<V>> build();

   public abstract static class Keys<K0> {
      <V0 extends Base<V0>> TestDiamondArgumentConflict<K0, V0> baseValues(Key<V0> key) {
         return new TestDiamondArgumentConflict<K0, V0>() {
            @SuppressWarnings({"unchecked", "rawtypes"})
            <K extends K0, V extends V0> Supplier<Box<V>> build() {
               Supplier<Box<V>> factory = (Supplier) new BaseSupplier<V0>(key);
               return wrap(factory);
            }
         };
      }

      <V0 extends Base<V0>> Supplier<Box<V0>> direct(Key<V0> key) {
         Supplier<Box<V0>> factory = new BaseSupplier<>(key);
         return wrap(factory);
      }
   }
}
