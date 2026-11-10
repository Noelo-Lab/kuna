package pkg;

public class TestLabelledContinueOuterEnd {
   public static int labelled(int[] a) {
      int r = 0;

      for (int x : a) {
         for (int y : a) {
            if (x == y) {
               break;
            }

            if (x > y) {
               return r;
            }

            r += y;
         }
      }

      return r;
   }

   public static int whileTrue(int n) {
      int r = 0;

      for (int i = 0; i < n; i++) {
         int j = 0;

         while (++j <= i) {
            r += j;
         }
      }

      return r;
   }

   public static int doWhile(int[] a) {
      int r = 0;
      int i = 0;

      while (i < a.length) {
         int v = a[i++];

         while (v != 0) {
            r += v;
            if (--v <= 2) {
               r += 100;
               break;
            }
         }
      }

      return r;
   }
}
