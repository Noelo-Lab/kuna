package pkg;

public class TestNarrowConstLocal {
   static final int DOUBLE = 15;
   static final int REAL = 14;
   byte[] bytes = new byte[4];
   char last;

   static boolean readIf(String s) {
      return s.isEmpty();
   }

   static int type(int t, int precision) {
      return t + precision;
   }

   int parseFloatType(String s) {
      int type = DOUBLE;
      int precision = 53;
      if (readIf(s)) {
         type = REAL;
         precision = 24;
      }
      return type(type, precision);
   }

   void store(int n) {
      byte b = 7;
      if (n > 0) {
         b = 9;
      }
      this.bytes[0] = b;
      char c = 'q';
      if (n > 1) {
         c = 'r';
      }
      this.last = c;
      short sh = 300;
      int sum = sh + n;
      System.out.println(sum);
   }
}
