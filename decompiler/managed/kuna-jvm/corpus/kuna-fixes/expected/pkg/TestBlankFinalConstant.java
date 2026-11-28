package pkg;

public class TestBlankFinalConstant {
   static final int CONSTANT = 1;
   static final int FIRST;
   static final String NAME;
   static final long WIDE;

   static {
      FIRST = 2;
      NAME = "n";
      WIDE = 3L;
   }

   static final Object BOXED = "o";
   static final String ABSENT = null;
   static int counter;
   static final int SUM = 1 + FIRST;
   static final String LATE;
   final int instance;
   final int initialized = 5;
   final String text;
   int plain = 6;

   TestBlankFinalConstant() {
      this.instance = 4;
      this.text = "t";
   }

   int sum() {
      return this.instance + this.initialized + this.text.length() + this.plain;
   }

   static String read() {
      return NAME + FIRST + WIDE + LATE + "1";
   }

   static {
      counter = SUM;
      LATE = "late";
   }
}
