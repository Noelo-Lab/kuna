package pkg;

public class TestLambdaInstantiatedType {
   interface Fn<A, R> {
      R apply(A a);
   }

   interface Sink<T> {
      void put(T t);
   }

   static class Box {
      int size() {
         return 1;
      }
   }

   static void locals() {
      Fn<String, Object> length = s -> s.length() + 1;
      Sink<Box> sink = Box::size;
      Fn<Box, Integer> sizes = b -> b.size();
   }

   static <T> T receiver(Fn<T, T> id, T value) {
      T first = ((Fn<Fn<T, T>, T>) f -> f.apply(value)).apply(id);
      return first;
   }

   static Object supplied() {
      return ((Fn<Object, String>) o -> "x").apply(null);
   }
}
