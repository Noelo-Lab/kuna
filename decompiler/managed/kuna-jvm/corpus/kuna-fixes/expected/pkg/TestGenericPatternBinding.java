package pkg;

public class TestGenericPatternBinding {
   static <V> TestGenericPatternBinding.Box<V> unwrap(TestGenericPatternBinding.Source<TestGenericPatternBinding.Box<V>> source) {
      return source instanceof TestGenericPatternBinding.Box<V> box ? box : source.get();
   }

   static <V> TestGenericPatternBinding.Box<V> open(TestGenericPatternBinding.Source<TestGenericPatternBinding.Box<V>> source) {
      if (source instanceof TestGenericPatternBinding.Box<V> box) {
         return box;
      }

      return source.get();
   }

   static final class Box<V> implements TestGenericPatternBinding.Source<TestGenericPatternBinding.Box<V>> {
      public TestGenericPatternBinding.Box<V> get() {
         return this;
      }
   }

   interface Source<T> {
      T get();
   }
}
