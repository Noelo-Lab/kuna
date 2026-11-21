package pkg;

public class TestSwitchAssignNotYield {
   static int skip(TestSwitchAssignNotYield.Time t) {
      return 0;
   }

   public static TestSwitchAssignNotYield.Time apply(String math, TestSwitchAssignNotYield.Time t, boolean up) {
      int i = 0;

      while (i < math.length()) {
         boolean round = math.charAt(i++) == '/';
         char unit = math.charAt(i++);
         switch (unit) {
            case 'H':
            case 'h':
               if (round) {
                  t = t.truncate();
                  if (up) {
                     t = t.plus(1L);
                  }
               } else {
                  t = t.plus(2L);
               }
               break;
            case 'm':
               if (round) {
                  t = t.truncate().truncate();
                  if (up) {
                     t = t.plus(3L);
                  }
               } else {
                  t = t.plus(4L);
               }
               break;
            default:
               throw new IllegalStateException("unit");
         }

         i += skip(t);
      }

      return t;
   }

   static int swap(int v) {
      return v >>> 16 | v << 16;
   }

   public static int encode(int mode, int value) {
      switch (mode) {
         case 0:
            break;
         case 1:
            value <<= 8;
            break;
         case 2:
            value = swap(value);
            break;
         default:
            throw new IllegalStateException("mode");
      }

      return value * 2;
   }

   public static double first(int mode, double[] scores, boolean[] hits) {
      double factor = 1.0;
      switch (mode) {
         case 0:
            for (int i = 0; i < scores.length; i++) {
               if (hits[i]) {
                  factor = scores[i];
                  break;
               }
            }
            break;
         case 1:
            factor = scores[0];
            break;
         default:
            throw new IllegalStateException("mode");
      }

      return factor;
   }

   interface Time {
      TestSwitchAssignNotYield.Time truncate();

      TestSwitchAssignNotYield.Time plus(long var1);
   }
}
