package pkg;

public class TestCurriedLambdaRaw {
   interface Fn<A, B> {
      B apply(A a);
   }

   static void take(Fn<String, Fn<String, Integer>> f) {
   }

   int sites(String x, int k) {
      Fn<String, Fn<String, Integer>> curried = a -> b -> a.length() + b.length() + k;
      Fn<String, Fn<String, Fn<String, Integer>>> deeper = a -> b -> c -> a.length() + b.length() + c.length();
      take(a -> b -> a.length() + b.indexOf(a));
      Fn<String, Fn<String, Integer>> refs = a -> String::length;
      Fn<String, Fn<String, Integer>> block = a -> {
         int n = a.length();
         return b -> b.length() + n;
      };
      return curried.apply(x).apply(x) + deeper.apply(x).apply(x).apply(x) + block.apply(x).apply(x) + refs.apply(x).apply(x);
   }
}
