package pkg;

public class TestRecordPatternInTry {
   TestRecordPatternInTry.Shape load(String key) throws TestRecordPatternInTry.Missing {
      return null;
   }

   // $VF: Duplicated exception handlers to handle obfuscated exceptions
   public Object describe(String key) {
      try {
         TestRecordPatternInTry.Shape s = this.load(key);
         if (s instanceof TestRecordPatternInTry.Circle(int r)) {
            return r * 2;
         } else {
            return s instanceof TestRecordPatternInTry.Label(String t) ? t : null;
         }
      } catch (TestRecordPatternInTry.Missing e) {
         return e;
      }
   }

   public int measure(String key) {
      int size = 0;

      try {
         TestRecordPatternInTry.Shape s = this.load(key);
         if (s instanceof TestRecordPatternInTry.Circle(int r)) {
            size = r;
         } else if (s instanceof TestRecordPatternInTry.Label(String t)) {
            size = t.length();
         }
      } catch (TestRecordPatternInTry.Missing e) {
         size = -1;
      }

      return size;
   }

   record Circle(int radius) implements TestRecordPatternInTry.Shape {
   }

   record Label(String text) implements TestRecordPatternInTry.Shape {
   }

   static class Missing extends Exception {
   }

   sealed interface Shape permits TestRecordPatternInTry.Circle, TestRecordPatternInTry.Label {
   }
}
