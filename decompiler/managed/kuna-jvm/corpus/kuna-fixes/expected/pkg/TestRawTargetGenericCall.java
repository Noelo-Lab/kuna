package pkg;

public class TestRawTargetGenericCall {
   static void takeRaw(TestRawTargetGenericCall.Cmp var0) {
   }

   int sites(int var1) {
      TestRawTargetGenericCall.Box var2 = new TestRawTargetGenericCall.Box();
      var2.sort(TestRawTargetGenericCall.Cmp.comparing((String var1x) -> var1x.length() + var1));
      takeRaw(TestRawTargetGenericCall.Cmp.comparing((String var0) -> var0.trim()));
      TestRawTargetGenericCall.Cmp var3 = TestRawTargetGenericCall.Cmp.comparing((String var0) -> var0.trim());
      var2.sort(var3);
      TestRawTargetGenericCall.Box var4 = new TestRawTargetGenericCall.Box();
      var4.sort(TestRawTargetGenericCall.Cmp.comparing((int[] var0) -> var0.length));
      return ((String)var2.held).length();
   }

   static class Box<T> {
      T held;

      void sort(TestRawTargetGenericCall.Cmp<? super T> var1) {
      }
   }

   interface Cmp<T> {
      int compare(T var1, T var2);

      static <T, U extends Comparable<? super U>> TestRawTargetGenericCall.Cmp<T> comparing(TestRawTargetGenericCall.Fn<? super T, ? extends U> var0) {
         return (var0x, var1) -> 0;
      }
   }

   interface Fn<A, B> {
      B apply(A var1);
   }
}
