package pkg;

public class TestLabelledContinueOuterEnd {
   public static int labelled(int[] a) {
      int r = 0;
      outer:
      for (int x : a) {
         for (int y : a) {
            if (x == y) {
               continue outer;
            }
            if (x > y) {
               break outer;
            }
            r += y;
         }
      }
      return r;
   }

   public static int whileTrue(int n) {
      int r = 0;
      outer:
      for (int i = 0; i < n; i++) {
         int j = 0;
         while (true) {
            j++;
            if (j > i) {
               continue outer;
            }
            r += j;
         }
      }
      return r;
   }

   public static int doWhile(int[] a) {
      int r = 0;
      int i = 0;
      outer:
      while (i < a.length) {
         int v = a[i++];
         do {
            if (v == 0) {
               continue outer;
            }
            r += v;
            v--;
         } while (v > 2);
         r += 100;
      }
      return r;
   }
}
