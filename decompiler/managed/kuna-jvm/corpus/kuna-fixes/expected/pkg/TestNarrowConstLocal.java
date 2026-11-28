package pkg;

public class TestNarrowConstLocal {
   static final int DOUBLE = 15;
   static final int REAL = 14;
   byte[] bytes = new byte[4];
   char last;

   static boolean readIf(String var0) {
      return var0.isEmpty();
   }

   static int type(int var0, int var1) {
      return var0 + var1;
   }

   int parseFloatType(String var1) {
      int var2 = 15;
      int var3 = 53;
      if (readIf(var1)) {
         var2 = 14;
         var3 = 24;
      }

      return type(var2, var3);
   }

   void store(int var1) {
      byte var2 = 7;
      if (var1 > 0) {
         var2 = 9;
      }

      this.bytes[0] = var2;
      char var3 = 'q';
      if (var1 > 1) {
         var3 = 'r';
      }

      this.last = var3;
      int var4 = 300;
      int var5 = var4 + var1;
      System.out.println(var5);
   }
}
