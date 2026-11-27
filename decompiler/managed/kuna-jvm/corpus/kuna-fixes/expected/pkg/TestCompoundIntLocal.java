package pkg;

public class TestCompoundIntLocal {
   static int lowestBit(long var0) {
      if (var0 == 0L) {
         return -1;
      }

      int var2 = 0;
      int var3 = 32;
      long var4 = -1L;

      do {
         var4 >>>= var3;
         if ((var4 & var0) == 0L) {
            var2 += var3;
            var0 >>>= var3;
         }

         var3 >>= 1;
      } while (var3 != 0);

      return var2;
   }

   static int flags(int[] var0) {
      int var1 = 0;

      for (int var5 : var0) {
         var1 |= var5;
      }

      int var6 = 1;
      var6 *= var0.length;
      return var1 + var6;
   }

   static void take(byte var0) {
   }

   static void take(int var0) {
   }

   static void narrow(int var0) {
      byte var1 = 0;
      var1 = (byte)(var1 + var0);
      take(var1);
      int var2 = 0;
      var2 -= var0;
      take(var2);
   }
}
