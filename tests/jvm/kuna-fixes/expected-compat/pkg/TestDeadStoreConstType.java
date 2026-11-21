package pkg;

public class TestDeadStoreConstType {
   static int length(int base, byte flags) {
      int length = base;
      int override = (flags & 112) >> 4;
      if (override != 0 && override < length) {
         length = override;
      } else {
         int var4 = false;
      }

      return length;
   }

   static int count(int[] a) {
      int n = 0;
      int last = 1;

      for (int v : a) {
         if (v > 0) {
            n++;
         } else {
            int var7 = true;
         }
      }

      return n;
   }
}
