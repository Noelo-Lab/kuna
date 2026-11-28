package pkg;

public class TestCompoundIntLocal {
   static int lowestBit(long val) {
      if (val == 0) {
         return -1;
      }
      int res = 0;
      int sz = 32;
      long mask = -1;
      do {
         mask >>>= sz;
         if ((mask & val) == 0) {
            res += sz;
            val >>>= sz;
         }
         sz >>= 1;
      } while (sz != 0);
      return res;
   }

   static int flags(int[] bits) {
      int flags = 0;
      for (int bit : bits) {
         flags |= bit;
      }
      int count = 1;
      count *= bits.length;
      return flags + count;
   }

   static void take(byte b) {
   }

   static void take(int i) {
   }

   static void narrow(int x) {
      byte b = 0;
      b += x;
      take(b);
      int i = 0;
      i -= x;
      take(i);
   }
}
