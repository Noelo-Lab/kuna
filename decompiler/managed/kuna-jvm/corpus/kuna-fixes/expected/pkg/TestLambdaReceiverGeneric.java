package pkg;

public class TestLambdaReceiverGeneric {
   static String supplied() {
      String s = ((TestLambdaReceiverGeneric.Source<String>)(() -> "x")).get();
      return s;
   }

   static Integer size(String text) {
      return text.length();
   }

   static int applied(String text) {
      return ((TestLambdaReceiverGeneric.Mapper<String, Integer>)(TestLambdaReceiverGeneric::size)).apply(text);
   }

   static <T> T chosen(TestLambdaReceiverGeneric.Source<T> source) {
      T first = ((TestLambdaReceiverGeneric.Mapper<TestLambdaReceiverGeneric.Source, T>)(s -> (T)s.get())).apply(source);
      return first;
   }

   static Object erased() {
      return ((TestLambdaReceiverGeneric.Source)(() -> "y")).get();
   }

   interface Mapper<A, R> {
      R apply(A var1);
   }

   interface Source<T> {
      T get();
   }
}
