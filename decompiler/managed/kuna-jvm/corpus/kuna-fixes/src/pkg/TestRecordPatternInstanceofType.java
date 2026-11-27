package pkg;

public class TestRecordPatternInstanceofType {
   record Box(Object o) {
   }

   static boolean positive(Object o) {
      return o instanceof Box(Integer i) && i > 0;
   }

   static int length(Object o) {
      if (o instanceof Box(String s)) {
         return s.length();
      }
      return -1;
   }
}
