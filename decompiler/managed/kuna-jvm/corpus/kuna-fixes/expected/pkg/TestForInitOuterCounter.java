package pkg;

public class TestForInitOuterCounter {
   static int twoBlocks(int[] a, int[] b) {
      int i;
      int s = 0;
      int n = a.length;

      for (i = 0; i < n; i++) {
         s += a[i];
      }

      s += i;
      n = b.length;

      for (i = 0; i < n; i++) {
         s += b[i];
      }

      return s + i;
   }
}
