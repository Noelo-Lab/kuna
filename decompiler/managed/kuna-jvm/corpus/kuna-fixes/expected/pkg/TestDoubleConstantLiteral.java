package pkg;

public class TestDoubleConstantLiteral {
   static double toDouble(long state) {
      return ((int)(state >>> 33) + 1) / 2.147483648E9;
   }

   static double gbFrac(long bytes) {
      return bytes / 1.073741824E9;
   }

   static boolean below(long bytes) {
      return bytes < 1.099511627776E12;
   }

   static Object boxed() {
      return 1.073741824E9;
   }

   static String text(StringBuilder sb) {
      return sb.append(1.125899906842624E15).toString();
   }

   static double scaled(double d) {
      return d * 1.073741824E9;
   }

   static double max(double d) {
      return d * 3.4028234663852886E38;
   }

   static double half() {
      return 0.5;
   }
}
