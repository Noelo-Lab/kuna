package pkg;

public class TestDeadStoreConstType {
   static int length(int base, byte flags) {
      int length = base;
      int override = (flags & 0x70) >> 4;
      if (override != 0 && override < length) {
         length = override;
      } else {
         override = 0;
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
            last = 1;
         }
      }
      return n;
   }
}
