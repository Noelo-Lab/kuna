package pkg;

public class TestCompoundShiftCount {
   public static int lowestBit(long val) {
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

   public static long rotate(long v, int n) {
      int s = n & 63;
      long r = v;
      r <<= s;
      return r | v >>> 64 - s;
   }
}
