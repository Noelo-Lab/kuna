package pkg;

public class TestRawTargetLambda<E extends CharSequence> {
   static <T> TestRawTargetLambda.Box<T> boxOf(T var0) {
      return new TestRawTargetLambda.Box<>(var0, var0x -> "b");
   }

   static void takeRaw(TestRawTargetLambda.Fn var0) {
   }

   public String receiver(String var1, int var2) {
      TestRawTargetLambda.Box var3 = boxOf(var1);
      var3.removeIf((TestRawTargetLambda.Pred<String>)(var1x -> var1x.length() > var2));
      var3.removeIf((TestRawTargetLambda.Pred<String>)(var1x -> var1x.hashCode() > var2));
      return (String)var3.value;
   }

   public E typeVariable(E var1) {
      TestRawTargetLambda.Box var2 = boxOf(var1);
      var2.removeIf((TestRawTargetLambda.Pred<CharSequence>)(var0 -> var0.length() > 0));
      return (E)var2.value;
   }

   public int[] arrays(int[] var1) {
      TestRawTargetLambda.Box var2 = boxOf(var1);
      var2.removeIf((TestRawTargetLambda.Pred<int[]>)(var0 -> var0.length > 0));
      return (int[])var2.value;
   }

   public String constructor(String var1) {
      TestRawTargetLambda.Box var2 = new TestRawTargetLambda.Box<>(var1, var0 -> var0.trim());
      return (String)var2.value;
   }

   public int reference(String var1) {
      TestRawTargetLambda.Box var2 = boxOf(var1);
      TestRawTargetLambda.Box var3 = var2.map((TestRawTargetLambda.Fn<String, Integer>)(String::length));
      return ((String)var2.value).length() + var3.hashCode();
   }

   public int chain(String var1) {
      TestRawTargetLambda.Box var2 = boxOf(var1);
      Integer[] var3 = (Integer[])var2.map((TestRawTargetLambda.Fn<String, Integer>)(var0 -> var0.length())).toArray(Integer[]::new);
      return var3.length + ((String)var2.value).length();
   }

   public void declared() {
      takeRaw((TestRawTargetLambda.Fn<String, Integer>)(var0 -> var0.length()));
      takeRaw(var0 -> var0.hashCode());
   }

   public void lambdaParameter() {
      TestRawTargetLambda.Bi<TestRawTargetLambda.Box, Integer> var1 = (var0, var1x) -> {
         Integer[] var2 = (Integer[])var0.map((TestRawTargetLambda.Fn<String, Integer>)(var1xx -> var1xx.length() + var1x)).toArray(Integer[]::new);
      };
   }

   interface Bi<A, B> {
      void accept(A var1, B var2);
   }

   static class Box<T> {
      T value;

      Box(T var1, TestRawTargetLambda.Fn<T, String> var2) {
         this.value = var1;
      }

      boolean removeIf(TestRawTargetLambda.Pred<? super T> var1) {
         return false;
      }

      <R> TestRawTargetLambda.Box<R> map(TestRawTargetLambda.Fn<? super T, ? extends R> var1) {
         return null;
      }

      <A> A[] toArray(TestRawTargetLambda.Gen<A[]> var1) {
         return (A[])((Object[])var1.make(0));
      }
   }

   interface Fn<A, R> {
      R apply(A var1);
   }

   interface Gen<A> {
      A make(int var1);
   }

   interface Pred<T> {
      boolean test(T var1);
   }
}
