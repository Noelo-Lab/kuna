package pkg;

public class TestIncrementBeforeCondition {
   static int compound(int i, int j) {
      if (j < 0) {
         j -= 1;
         if (i > 3) {
            if (j == -2) {
               j = 1;
            }
         }
      }
      return j;
   }

   static int either(int i, int j) {
      j++;
      if (i > 3 || j == 2) {
         return j;
      }
      return -j;
   }

   static int single(int i, int j) {
      j--;
      if (j == 3) {
         return i;
      }
      return j;
   }
}
