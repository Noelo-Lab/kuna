package pkg;

public class TestForUpdateList {
   static int next(int x) {
      return x + 1;
   }

   static int two(int[] a) {
      int s = 0;

      for (int i = 0, j = a.length - 1; i < j; i++, j--) {
         s += a[i] - a[j];
      }

      return s;
   }

   static int three(int[] a) {
      int s = 0;

      for (int i = 0, j = a.length - 1, k = 0; i < j; i++, j--, k += 2) {
         s += a[i] - a[j] + k;
      }

      return s;
   }

   static int withContinue(int[] a) {
      int s = 0;

      for (int i = 0, j = a.length - 1; i < j; i++, j--) {
         if (a[i] < 0) {
            continue;
         }

         s += a[i] - a[j];
      }

      return s;
   }

   static int call(int[] a) {
      int s = 0;
      int n = 0;

      for (int i = 0; i < a.length; n = next(n), i++) {
         s += a[i] + n;
      }

      return s + n;
   }

   static int readByUpdate(int[] a, int bits) {
      int s = 0;

      for (int i = 0, shift = bits; shift <= 32; i++, shift += bits) {
         s += a[i] << shift;
      }

      return s;
   }

   static int bodyIncrement(int[] a) {
      int s = 0;
      int n = 0;

      for (int i = 0; i < a.length; i++) {
         s += a[i];
         n++;
      }

      return s + n;
   }

   static int whileLoop(int[] a) {
      int s = 0;
      int i = 0;
      int j = a.length - 1;

      while (i < j) {
         s += a[i] - a[j];
         i++;
         j--;
      }

      return s;
   }

   static int oneLine(int[] a) {
      int s = 0;

      for (int i = 0, j = a.length - 1; i < j; j--) {
         s += a[i] - a[j];
         i++;
      }

      return s;
   }
}
