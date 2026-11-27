package pkg;

public class TestTernaryIntLocal {
   static int decode(TestTernaryIntLocal.Source var0) {
      int var1 = 0;
      int var2 = 0;

      while (true) {
         int var3 = var0.next();
         if (var3 == 0) {
            break;
         }

         if (var3 == 1) {
            var1 = var0.next();
         }

         var2 += var3;
      }

      int var4 = var1 != 0 ? var1 : var2;
      return var4;
   }

   static int orDefault(TestTernaryIntLocal.Source var0) {
      int var1 = 0;

      while (var0.next() == 1) {
         if (var0.next() == 2) {
            var1 = var0.next();
         }
      }

      int var2 = var1 != 0 ? var1 : 7;
      return var2;
   }

   interface Source {
      int next();
   }
}
