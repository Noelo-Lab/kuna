package pkg;

public class TestLambdaReceiverGeneric {
   interface Source<T> {
      T get();
   }

   interface Mapper<A, R> {
      R apply(A a);
   }

   static String supplied() {
      String s = ((Source<String>) () -> "x").get();
      return s;
   }

   static Integer size(String text) {
      return text.length();
   }

   static int applied(String text) {
      return ((Mapper<String, Integer>) TestLambdaReceiverGeneric::size).apply(text);
   }

   static <T> T chosen(Source<T> source) {
      T first = ((Mapper<Source<T>, T>) s -> s.get()).apply(source);
      return first;
   }

   static Object erased() {
      return ((Source<?>) () -> "y").get();
   }
}
