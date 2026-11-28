package pkg;

public class TestBlankFinalConstant {
   static final int CONSTANT = 1;
   static final int FIRST;
   static final String NAME;
   static final long WIDE;
   static final Object BOXED;
   static final String ABSENT;
   static int counter;

   static {
      FIRST = 2;
      NAME = "n";
      WIDE = 3L;
      BOXED = "o";
      ABSENT = null;
   }

   static final int SUM = CONSTANT + FIRST;
   static final String LATE;

   static {
      counter = SUM;
      LATE = "late";
   }

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
      return NAME + FIRST + WIDE + LATE + CONSTANT;
   }
}
