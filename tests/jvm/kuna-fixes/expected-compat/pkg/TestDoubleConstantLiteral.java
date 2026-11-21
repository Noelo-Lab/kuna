package pkg;

public class TestDoubleConstantLiteral {
   static double toDouble(long state) {
      return ((int)(state >>> 33) + 1) / 2.1474836E9F;
   }

   static double gbFrac(long bytes) {
      return bytes / 1.0737418E9F;
   }

   static boolean below(long bytes) {
      return bytes < 1.0995116E12F;
   }

   static Object boxed() {
      return 1.0737418E9F;
   }

   static String text(StringBuilder sb) {
      return sb.append(1.1258999E15F).toString();
   }

   static double scaled(double d) {
      return d * 1.0737418E9F;
   }

   static double max(double d) {
      return d * Float.MAX_VALUE;
   }

   static double half() {
      return 0.5;
   }
}
