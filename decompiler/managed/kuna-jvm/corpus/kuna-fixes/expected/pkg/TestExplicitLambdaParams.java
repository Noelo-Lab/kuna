package pkg;

public class TestExplicitLambdaParams {
   static void wildcard(TestExplicitLambdaParams.Fn<?, ?> fn) {
   }

   static <T> void lonely(TestExplicitLambdaParams.Fn<T, ?> fn) {
   }

   static <T> T fixed(T value, TestExplicitLambdaParams.Fn<T, ?> fn) {
      return value;
   }

   static void bounded(TestExplicitLambdaParams.Fn<? super String, ?> fn) {
   }

   static void calls() {
      wildcard((String s) -> s.length());
      lonely((String s) -> s.length());
      fixed("x", s -> s.length());
      bounded(s -> s.length());
      TestExplicitLambdaParams.Fn<?, ?> local = (String s) -> s.isEmpty();
   }

   interface Fn<A, R> {
      R apply(A var1);
   }
}
