package pkg;

public class TestForUpdateInElse {
   static boolean scan(int[] cells, int limit, int mode) {
      int i;
      for (i = 0; i < limit; i++) {
         if (cells[i] != 0) {
            if (cells[i] > 5 && cells[i] > 7) {
               if (mode != 0) {
                  return false;
               }
               break;
            } else {
               return false;
            }
         }
      }

      return i > 0;
   }
}
