package pkg;

public class TestMethodRefWildcardReceiver {
   private Pipe<Class<?>> types;

   Pipe<Object> objects(Factory<?> factory) {
      return this.types.map(factory::create);
   }

   Pipe<Shape<?>> shapes(ShapeFactory<?> factory) {
      return this.types.map(factory::create);
   }

   Pipe<Object> objectsOf(Factory<String> factory) {
      return this.types.map(factory::create);
   }

   interface Fn<T, R> {
      R apply(T t);
   }

   interface Pipe<T> {
      <R> Pipe<R> map(Fn<? super T, ? extends R> mapper);
   }

   interface Shape<T> {
   }

   static class Factory<T> {
      T create(Class<?> type) {
         return null;
      }
   }

   static class ShapeFactory<S extends Shape<?>> {
      S create(Class<?> type) {
         return null;
      }
   }
}
