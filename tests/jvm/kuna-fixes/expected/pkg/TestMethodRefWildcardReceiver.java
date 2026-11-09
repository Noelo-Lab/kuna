package pkg;

public class TestMethodRefWildcardReceiver {
   private TestMethodRefWildcardReceiver.Pipe<Class<?>> types;

   TestMethodRefWildcardReceiver.Pipe<Object> objects(TestMethodRefWildcardReceiver.Factory<?> factory) {
      return this.types.map(factory::create);
   }

   TestMethodRefWildcardReceiver.Pipe<TestMethodRefWildcardReceiver.Shape<?>> shapes(TestMethodRefWildcardReceiver.ShapeFactory<?> factory) {
      return this.types.map(factory::create);
   }

   TestMethodRefWildcardReceiver.Pipe<Object> objectsOf(TestMethodRefWildcardReceiver.Factory<String> factory) {
      return this.types.map(factory::create);
   }

   static class Factory<T> {
      T create(Class<?> type) {
         return null;
      }
   }

   interface Fn<T, R> {
      R apply(T var1);
   }

   interface Pipe<T> {
      <R> TestMethodRefWildcardReceiver.Pipe<R> map(TestMethodRefWildcardReceiver.Fn<? super T, ? extends R> var1);
   }

   interface Shape<T> {
   }

   static class ShapeFactory<S extends TestMethodRefWildcardReceiver.Shape<?>> {
      S create(Class<?> type) {
         return null;
      }
   }
}
