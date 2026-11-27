package pkg;

public class TestRecordPatternInstanceofType {
   static boolean positive(Object var0) {
      return var0 instanceof TestRecordPatternInstanceofType.Box(Integer var1) && var1 > 0;
   }

   static int length(Object var0) {
      if (var0 instanceof TestRecordPatternInstanceofType.Box(String var2)) {
         return var2.length();
      }

      return -1;
   }

   record Box(Object o) {
   }
}
