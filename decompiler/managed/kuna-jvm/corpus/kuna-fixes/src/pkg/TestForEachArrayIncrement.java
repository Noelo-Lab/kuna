package pkg;

public class TestForEachArrayIncrement {
   static Object source;

   public void catchBodyIncrement(int[][] a) {
      for (int[] row : a) {
         switch (row.length) {
            case 0:
               try {
                  Object o = source;
               } catch (Exception e) {
                  a[0][0]++;
               } finally {
               }
         }
      }
   }

   public int conditionalIncrement(int[] a, boolean c) {
      int s = 0;
      int[] arr = a;
      int len = arr.length;
      int i = 0;
      while (i < len) {
         int x = arr[i];
         s += x;
         if (c) {
            i++;
         }
      }
      return s;
   }
}
