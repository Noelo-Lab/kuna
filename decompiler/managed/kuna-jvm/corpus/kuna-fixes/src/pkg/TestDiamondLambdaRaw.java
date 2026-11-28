package pkg;

public class TestDiamondLambdaRaw {
   interface Fn<A, B> {
      B apply(A a);
   }

   static class Holder<T> {
      final Fn<T, Integer> f;

      Holder(Fn<T, Integer> f) {
         this.f = f;
      }

      Holder(Fn<T, Integer> f, T seed) {
         this.f = f;
      }
   }

   static void take(Holder h) {
   }

   int sites(int k) {
      Holder<String> h = new Holder<>(s -> s.length() + k);
      Holder<String> seeded = new Holder<>(s -> s.length(), "v");
      take(new Holder<>((String s) -> s.trim().length()));
      return h.hashCode() + seeded.hashCode();
   }
}
