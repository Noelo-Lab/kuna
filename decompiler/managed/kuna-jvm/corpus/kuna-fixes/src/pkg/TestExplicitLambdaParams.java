package pkg;

public class TestExplicitLambdaParams {
   interface Fn<A, R> {
      R apply(A a);
   }

   static void wildcard(Fn<?, ?> fn) {
   }

   static <T> void lonely(Fn<T, ?> fn) {
   }

   static <T> T fixed(T value, Fn<T, ?> fn) {
      return value;
   }

   static void bounded(Fn<? super String, ?> fn) {
   }

   static void calls() {
      wildcard((String s) -> s.length());
      lonely((String s) -> s.length());
      fixed("x", s -> s.length());
      bounded(s -> s.length());
      Fn<?, ?> local = (String s) -> s.isEmpty();
   }
}
