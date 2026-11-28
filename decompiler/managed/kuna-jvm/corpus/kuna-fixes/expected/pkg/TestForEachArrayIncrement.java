package pkg;

public class TestForEachArrayIncrement {
   static Object source;

   public void catchBodyIncrement(int[][] var1) {
      for (int[] var5 : var1) {
         switch (var5.length) {
            case 0:
               try {
                  Object var6 = source;
               } catch (Exception var10) {
                  var1[0][0]++;
               } finally {
                  ;
               }
         }
      }
   }

   public int conditionalIncrement(int[] var1, boolean var2) {
      int var3 = 0;
      int[] var4 = var1;
      int var5 = var4.length;
      int var6 = 0;

      while (var6 < var5) {
         int var7 = var4[var6];
         var3 += var7;
         if (var2) {
            var6++;
         }
      }

      return var3;
   }
}
