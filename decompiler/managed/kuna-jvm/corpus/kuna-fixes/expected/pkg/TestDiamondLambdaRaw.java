package pkg;

public class TestDiamondLambdaRaw {
   static void take(TestDiamondLambdaRaw.Holder var0) {
   }

   int sites(int var1) {
      TestDiamondLambdaRaw.Holder var2 = new TestDiamondLambdaRaw.Holder<>((String var1x) -> var1x.length() + var1);
      TestDiamondLambdaRaw.Holder var3 = new TestDiamondLambdaRaw.Holder<>(var0 -> var0.length(), "v");
      take(new TestDiamondLambdaRaw.Holder<>((String var0) -> var0.trim().length()));
      return var2.hashCode() + var3.hashCode();
   }

   interface Fn<A, B> {
      B apply(A var1);
   }

   static class Holder<T> {
      final TestDiamondLambdaRaw.Fn<T, Integer> f;

      Holder(TestDiamondLambdaRaw.Fn<T, Integer> var1) {
         this.f = var1;
      }

      Holder(TestDiamondLambdaRaw.Fn<T, Integer> var1, T var2) {
         this.f = var1;
      }
   }
}
