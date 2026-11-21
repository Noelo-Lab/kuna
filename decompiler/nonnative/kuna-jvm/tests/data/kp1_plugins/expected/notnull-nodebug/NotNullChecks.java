package p;

import org.jetbrains.annotations.NotNull;

public class NotNullChecks {
   public void one(@NotNull String var1) {
      System.out.println(var1);
   }

   public static void wide(long var0, double var2, @NotNull Object var4, @NotNull String var5) {
      System.out.println(var0 + var2 + var4.hashCode() + var5);
   }

   public void notAnnotated(String var1, @NotNull String var2) {
      if (var1 == null) {
         $$$reportNull$$$0(3);
      }

      System.out.println(var1 + var2);
   }

   public void otherCall(@NotNull String var1) {
      if (var1 == null) {
         System.out.println("null");
      }

      System.out.println(var1);
   }

   public void notFirst(@NotNull String var1) {
      System.out.println(var1);
   }

   @NotNull
   public String ret(String var1) {
      return var1 + "!";
   }

   @NotNull
   public String retBranches(int var1, @NotNull String var2) {
      String var3 = var2.trim();
      if (var1 == 0) {
         return var3;
      }

      for (int var4 = 0; var4 < var1; var4++) {
         var3 = var3 + var4;
      }

      return var3;
   }

   @NotNull
   public String retThrow(String var1) {
      return var1.trim();
   }

   @NotNull
   public String retOther(String var1, String var2) {
      if (var2 == null) {
         $$$reportNull$$$0(9);
      }

      return var1;
   }

   private static void $$$reportNull$$$0(int var0) {
      throw new IllegalArgumentException(String.valueOf(var0));
   }

   public class Inner {
      public Inner(@NotNull String var2, @NotNull Object var3) {
         if (var2 == null) {
            NotNullChecks.$$$reportNull$$$0(10);
         }

         if (var3 == null) {
            NotNullChecks.$$$reportNull$$$0(11);
         }

         System.out.println(var2 + var3);
      }
   }
}
