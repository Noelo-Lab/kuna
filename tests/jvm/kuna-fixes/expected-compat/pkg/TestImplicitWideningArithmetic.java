package pkg;

public class TestImplicitWideningArithmetic {
   static long negatedProduct(int size, int count) {
      return -size * count;
   }

   static long negated(int a) {
      return -a;
   }

   static double negatedDouble(int a) {
      return -a;
   }

   static long remainderProduct(int a, int b, int c) {
      return a % b * c;
   }

   static long maskedShift(int a, int b, int n) {
      return (a & b) << n;
   }

   static long choiceProduct(boolean c, int a, int b, int d) {
      return (c ? a : b) * d;
   }

   static double remainder(int a, int b) {
      return a % b;
   }

   static long product(int a, int b) {
      return (long)a * b;
   }

   static long shifted(int a, int n) {
      return (long)a << n;
   }

   static long widenedResult(int a, int b) {
      long r = a * b;
      return r - a;
   }
}
