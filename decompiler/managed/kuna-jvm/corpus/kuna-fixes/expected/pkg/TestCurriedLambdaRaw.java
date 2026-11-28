package pkg;

public class TestCurriedLambdaRaw {
   static void take(TestCurriedLambdaRaw.Fn<String, TestCurriedLambdaRaw.Fn<String, Integer>> var0) {
   }

   int sites(String var1, int var2) {
      TestCurriedLambdaRaw.Fn<String, TestCurriedLambdaRaw.Fn> var3 = var1x -> (TestCurriedLambdaRaw.Fn<String, Integer>)(var2x -> var1x.length()
         + var2x.length()
         + var2);
      TestCurriedLambdaRaw.Fn<String, TestCurriedLambdaRaw.Fn> var4 = var0 -> (TestCurriedLambdaRaw.Fn<String, TestCurriedLambdaRaw.Fn>)(var1x -> (TestCurriedLambdaRaw.Fn<String, Integer>)(var2x -> var0.length()
         + var1x.length()
         + var2x.length()));
      take(var0 -> var1x -> var0.length() + var1x.indexOf(var0));
      TestCurriedLambdaRaw.Fn<String, TestCurriedLambdaRaw.Fn> var5 = var0 -> (TestCurriedLambdaRaw.Fn<String, Integer>)(String::length);
      TestCurriedLambdaRaw.Fn<String, TestCurriedLambdaRaw.Fn> var6 = var0 -> {
         int var1x = var0.length();
         return (TestCurriedLambdaRaw.Fn<String, Integer>)(var1xx -> var1xx.length() + var1x);
      };
      return (Integer)var3.apply(var1).apply(var1)
         + (Integer)((TestCurriedLambdaRaw.Fn)var4.apply(var1).apply(var1)).apply(var1)
         + (Integer)var6.apply(var1).apply(var1)
         + (Integer)var5.apply(var1).apply(var1);
   }

   interface Fn<A, B> {
      B apply(A var1);
   }
}
