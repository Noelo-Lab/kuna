package pkg;

public class TestLoopFirstIfExit {
   static int skipSmall(int[] v, int p) {
      int i = 0;
      int n = 0;

      while (n < 3) {
         while (true) {
            if (v[++i] >= p) {
               if (i > 5) {
                  break;
               }

               v[i] = 0;
               continue;
            }
         }

         n++;
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
                  break;
               }

               j--;
               continue;
            }
         }

         if (k <= j) {
            to = j - 1;
         } else {
            return i;
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
