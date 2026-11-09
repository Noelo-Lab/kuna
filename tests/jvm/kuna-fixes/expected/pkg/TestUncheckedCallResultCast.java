package pkg;

public class TestUncheckedCallResultCast {
   <T> TestUncheckedCallResultCast.Box<T> resolved(TestUncheckedCallResultCast.Kind<?> kind) {
      return (TestUncheckedCallResultCast.Box<T>)kind.any();
   }

   <T> T first(TestUncheckedCallResultCast.Kind<?> kind) {
      TestUncheckedCallResultCast.Box<T> box = (TestUncheckedCallResultCast.Box<T>)kind.any();
      return box.value;
   }

   TestUncheckedCallResultCast.Box<TestUncheckedCallResultCast.Shape> shapes(
      TestUncheckedCallResultCast.Factory<? extends TestUncheckedCallResultCast.Shape> factory
   ) {
      return (TestUncheckedCallResultCast.Box<TestUncheckedCallResultCast.Shape>)factory.make();
   }

   TestUncheckedCallResultCast.Box<TestUncheckedCallResultCast.Shape> chained(
      TestUncheckedCallResultCast.Factory<? extends TestUncheckedCallResultCast.Shape> factory
   ) {
      return (TestUncheckedCallResultCast.Box<TestUncheckedCallResultCast.Shape>)factory.make().self();
   }

   TestUncheckedCallResultCast.Box<String> exact(TestUncheckedCallResultCast.Kind<String> kind) {
      return kind.exact();
   }

   TestUncheckedCallResultCast.Box<TestUncheckedCallResultCast.Shape> generic(TestUncheckedCallResultCast.Kind<?> kind) {
      return kind.generic();
   }

   static class Box<V> {
      V value;

      TestUncheckedCallResultCast.Box<V> self() {
         return this;
      }
   }

   static class Factory<P> {
      TestUncheckedCallResultCast.Box<P> make() {
         return new TestUncheckedCallResultCast.Box<>();
      }
   }

   static class Kind<T> {
      TestUncheckedCallResultCast.Box<?> any() {
         return new TestUncheckedCallResultCast.Box();
      }

      TestUncheckedCallResultCast.Box<T> exact() {
         return new TestUncheckedCallResultCast.Box<>();
      }

      <R> TestUncheckedCallResultCast.Box<R> generic() {
         return new TestUncheckedCallResultCast.Box<>();
      }
   }

   static class Shape {
   }

   static class Shelf<B extends TestUncheckedCallResultCast.Shape> {
      TestUncheckedCallResultCast.Box<B> items() {
         return new TestUncheckedCallResultCast.Box<>();
      }

      TestUncheckedCallResultCast.Box<TestUncheckedCallResultCast.Shape> all() {
         return (TestUncheckedCallResultCast.Box)this.items();
      }
   }
}
