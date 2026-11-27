package pkg;

public class TestIncrementBeforeCondition {
   static int compound(int var0, int var1) {
      if (var1 < 0) {
         var1--;
         if (var0 > 3 && var1 == -2) {
            var1 = 1;
         }
      }

      return var1;
   }

   static int either(int var0, int var1) {
      var1++;
      if (var0 > 3 || var1 == 2) {
         return var1;
      }

      return -var1;
   }

   static int single(int var0, int var1) {
      if (--var1 == 3) {
         return var0;
      }

      return var1;
   }
}
