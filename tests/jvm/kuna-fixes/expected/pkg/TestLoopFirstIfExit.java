package pkg;

public class TestLoopFirstIfExit {
   static int skipSmall(int[] v, int p) {
      int i = 0;

      for (int n = 0; n < 3; n++) {
         while (true) {
            if (v[++i] >= p) {
               if (i > 5) {
                  break;
               }

               v[i] = 0;
               continue;
            }
         }
      }

      return i;
   }

   static int partition(int[] v, int p, int k) {
      int from = 0;
      int to = v.length;

      while (to - from > 3) {
         int i = from;
         int j = to;

         while (true) {
            if (v[++i] >= p) {
               if (i >= j) {
                  if (k <= j) {
                     to = j - 1;
                     break;
                  } else {
                     return i;
                  }
               }

               j--;
               continue;
            }
         }
      }

      return -1;
   }

   static int scan(int[] v, int p) {
      int i = 0;

      while (true) {
         if (v[++i] >= p) {
            if (i > 5) {
               return i;
            }

            v[i] = 0;
            continue;
         }
      }
   }
}
