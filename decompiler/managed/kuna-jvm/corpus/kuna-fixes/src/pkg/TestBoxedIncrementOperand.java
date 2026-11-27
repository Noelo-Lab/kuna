package pkg;

public class TestBoxedIncrementOperand {
   static void take(int value) {
   }

   static int sum() {
      Integer a = 3;
      return ++a + ++a;
   }

   static int scaled() {
      Integer b = 7;
      int r = --b * 2;
      return r + b;
   }

   static void argument() {
      Integer c = 1;
      take(++c);
      take(c);
   }
}
