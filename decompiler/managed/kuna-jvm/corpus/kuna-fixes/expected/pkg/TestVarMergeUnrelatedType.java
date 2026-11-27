package pkg;

public class TestVarMergeUnrelatedType {
   static Object src(int var0) {
      return var0;
   }

   static void sink(Object var0) {
      System.out.print(var0 + ";");
   }

   static void conditional(boolean var0, int var1) {
      Object var2 = var0 ? "s" : var1;
      sink(var2);
      Integer var3 = (Integer)src(var1);
      sink(var3 + 1);
   }

   static void branches(boolean var0, int var1) {
      Object var2;
      if (var0) {
         var2 = "s";
      } else {
         var2 = var1;
      }

      sink(var2);
      Integer var3 = (Integer)src(var1);
      sink(var3 + 1);
   }

   static void copy(boolean var0, int var1, String var2) {
      Object var3 = var2;
      if (var0) {
         var3 = var1;
      }

      sink(var3);
      Integer var4 = (Integer)src(var1);
      sink(var4 + 1);
   }
}
