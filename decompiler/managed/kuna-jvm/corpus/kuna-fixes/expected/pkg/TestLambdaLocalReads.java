package pkg;

public class TestLambdaLocalReads<E extends CharSequence> {
   E item;

   static int use(TestLambdaLocalReads.Fn<String, Integer> var0) {
      return 0;
   }

   public int fits(String var1, CharSequence var2, Object var3) {
      TestLambdaLocalReads.Fn<String, Integer> var4 = var0 -> var0.length();
      TestLambdaLocalReads.Fn var5 = var0 -> var0.length();
      return var4.apply(var1) + var4.apply((String)var3) + var4.apply(null) + (Integer)var5.apply("lit") + (Integer)var5.apply(var2);
   }

   public int arrays(String var1) {
      TestLambdaLocalReads.Bi<String, int[], Integer> var2 = (var0, var1x) -> var0.length() + var1x[0];
      return var2.apply(var1, new int[1]);
   }

   public int typeVariable(E var1) {
      TestLambdaLocalReads.Fn<CharSequence, Character> var2 = var0 -> var0.charAt(0);
      return var2.apply(var1) + var2.apply(this.item);
   }

   public int boxed(int var1) {
      TestLambdaLocalReads.Fn<Integer, Integer> var2 = var0 -> var0 + 1;
      return var2.apply(var1);
   }

   public int passed(String var1) {
      TestLambdaLocalReads.Fn var2 = var0 -> var0.hashCode();
      return use(var2) + (Integer)var2.apply(var1);
   }

   public int checked(TestLambdaLocalReads.Box<String> var1) {
      TestLambdaLocalReads.Fn<String, Integer> var2 = var0 -> var0.hashCode();
      return var2.apply(var1.get());
   }

   public int overloaded(CharSequence var1) {
      TestLambdaLocalReads.Ov var2 = var0 -> var0.hashCode();
      return var2.m(var1);
   }

   interface Bi<A, B, R> {
      R apply(A var1, B var2);
   }

   static class Box<T> {
      T value;

      T get() {
         return this.value;
      }
   }

   interface Fn<A, R> {
      R apply(A var1);
   }

   interface Ov<A> {
      int m(A var1);

      default int m(String var1) {
         return 0;
      }
   }
}
