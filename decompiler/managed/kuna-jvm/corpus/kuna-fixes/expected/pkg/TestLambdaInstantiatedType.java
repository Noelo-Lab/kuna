package pkg;

public class TestLambdaInstantiatedType {
   static void locals() {
      TestLambdaInstantiatedType.Fn<String, Object> var0 = var0x -> var0x.length() + 1;
      TestLambdaInstantiatedType.Sink<TestLambdaInstantiatedType.Box> var1 = TestLambdaInstantiatedType.Box::size;
      TestLambdaInstantiatedType.Fn<TestLambdaInstantiatedType.Box, Integer> var2 = var0x -> var0x.size();
   }

   static <T> T receiver(TestLambdaInstantiatedType.Fn<T, T> var0, T var1) {
      Object var2 = ((TestLambdaInstantiatedType.Fn<TestLambdaInstantiatedType.Fn, Object>)(var1x -> var1x.apply(var1))).apply(var0);
      return (T)var2;
   }

   static Object supplied() {
      return ((TestLambdaInstantiatedType.Fn<Object, String>)(var0 -> "x")).apply(null);
   }

   static class Box {
      int size() {
         return 1;
      }
   }

   interface Fn<A, R> {
      R apply(A var1);
   }

   interface Sink<T> {
      void put(T var1);
   }
}
