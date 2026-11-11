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

         t = switch (unit) {
            case 'H', 'h' -> {
               if (round) {
                  yield t.truncate();
                  if (up) {
                     yield t.plus(1L);
                  }
               } else {
                  yield t.plus(2L);
               }
            }
            case 'm' -> {
               if (round) {
                  yield t.truncate().truncate();
                  if (up) {
                     yield t.plus(3L);
                  }
               } else {
                  yield t.plus(4L);
               }
            }
            default -> throw new IllegalStateException("unit");
         };
         i += skip(t);
      }

      return t;
   }

   static int swap(int v) {
      return v >>> 16 | v << 16;
   }

   public static int encode(int mode, int value) {
      return switch (mode) {
         case 0 -> {
         }
         case 1 -> 8;
         case 2 -> swap(value);
         default -> throw new IllegalStateException("mode");
      } * 2;
   }

   public static double first(int mode, double[] scores, boolean[] hits) {
      double factor = 1.0;

      return switch (mode) {
         case 0 -> {
            for (int i = 0; i < scores.length; i++) {
               if (hits[i]) {
                  yield scores[i];
                  break;
               }
            }
         }
         case 1 -> scores[0];
         default -> throw new IllegalStateException("mode");
      };
   }

   interface Time {
      TestSwitchAssignNotYield.Time truncate();

      TestSwitchAssignNotYield.Time plus(long var1);
   }
}
