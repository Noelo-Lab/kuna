package pkg;

public abstract class TestDiamondArgumentConflict<K0, W0> {
   static <T> T wrap(T t) {
      return t;
   }

   abstract <K extends K0, V extends W0> TestDiamondArgumentConflict.Supplier<TestDiamondArgumentConflict.Box<V>> build();

   abstract static class Base<B extends TestDiamondArgumentConflict.Base<B>> {
   }

   static final class BaseSupplier<V extends TestDiamondArgumentConflict.Base<V>>
      implements TestDiamondArgumentConflict.Supplier<TestDiamondArgumentConflict.Box<V>> {
      final TestDiamondArgumentConflict.Key<V> key;

      BaseSupplier(TestDiamondArgumentConflict.Key<V> key) {
         this.key = key;
      }

      public TestDiamondArgumentConflict.Box<V> get() {
         return null;
      }
   }

   interface Box<T> {
   }

   static final class Key<T> {
   }

   public abstract static class Keys<K0> {
      <V0 extends TestDiamondArgumentConflict.Base<V0>> TestDiamondArgumentConflict<K0, V0> baseValues(final TestDiamondArgumentConflict.Key<V0> key) {
         return new TestDiamondArgumentConflict<K0, V0>() {
            @Override
            <K extends K0, V extends V0> TestDiamondArgumentConflict.Supplier<TestDiamondArgumentConflict.Box<V>> build() {
               TestDiamondArgumentConflict.Supplier<TestDiamondArgumentConflict.Box<V>> factory = new TestDiamondArgumentConflict.BaseSupplier<>(key);
               return wrap(factory);
            }
         };
      }

      <V0 extends TestDiamondArgumentConflict.Base<V0>> TestDiamondArgumentConflict.Supplier<TestDiamondArgumentConflict.Box<V0>> direct(
         TestDiamondArgumentConflict.Key<V0> key
      ) {
         TestDiamondArgumentConflict.Supplier<TestDiamondArgumentConflict.Box<V0>> factory = new TestDiamondArgumentConflict.BaseSupplier<>(key);
         return TestDiamondArgumentConflict.wrap(factory);
      }
   }

   interface Supplier<T> {
      T get();
   }
}
