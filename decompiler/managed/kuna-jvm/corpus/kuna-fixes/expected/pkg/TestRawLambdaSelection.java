package pkg;

public class TestRawLambdaSelection {
   TestRawLambdaSelection.Fn kept;

   static void over(TestRawLambdaSelection.Fn var0) {
   }

   static void over(TestRawLambdaSelection.Pred var0) {
   }

   static void takeRaw(TestRawLambdaSelection.Fn var0) {
   }

   void selected(int var1) {
      TestRawLambdaSelection.Runner var2 = new TestRawLambdaSelection.Runner();
      var2.run((TestRawLambdaSelection.Pred<String>)(var0 -> var0.isEmpty()));
      var2.run((TestRawLambdaSelection.Fn<String, String>)(var1x -> var1x.trim() + var1));
      over((TestRawLambdaSelection.Fn<String, Integer>)(var1x -> var1x.length() + var1));
      over((TestRawLambdaSelection.Pred<int[]>)(var1x -> var1x.length > var1));
      over((TestRawLambdaSelection.Pred)(var1x -> var1x.hashCode() > var1));
   }

   void assigned(int var1) {
      TestRawLambdaSelection.Fn var2 = (TestRawLambdaSelection.Fn<String, Integer>)(var1x -> var1x.length() + var1);
      takeRaw(var2);
      TestRawLambdaSelection.Fn var3 = (TestRawLambdaSelection.Fn<String, Integer>)(String::length);
      this.kept = var3;
      TestRawLambdaSelection.Fn var4 = var0 -> var0.hashCode();
      takeRaw(var4);
   }

   interface Fn<A, B> {
      B apply(A var1);
   }

   interface Pred<T> {
      boolean test(T var1);
   }

   static class Runner<T> {
      void run(TestRawLambdaSelection.Pred<T> var1) {
      }

      void run(TestRawLambdaSelection.Fn<T, String> var1) {
      }
   }
}
