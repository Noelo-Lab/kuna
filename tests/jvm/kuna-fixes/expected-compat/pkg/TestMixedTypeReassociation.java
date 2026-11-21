package pkg;

public class TestMixedTypeReassociation {
   int size;

   long offset(int ord) {
      return ord * this.size * 2L;
   }

   static long sum(int a, int b) {
      return a + b + 2L;
   }

   static long allLong(long x, long y, long z) {
      return x * y * z;
   }

   static long widenedFirst(int a, int b, int c) {
      return a * (long)b * c;
   }

   static int ints(int a, int b, int c) {
      return a * b * c;
   }
}
