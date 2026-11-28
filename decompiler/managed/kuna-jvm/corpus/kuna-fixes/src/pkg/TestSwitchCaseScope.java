package pkg;

public class TestSwitchCaseScope {
   public static double round(double x, int mode) {
      switch (mode) {
         case 0:
            return x;
         case 1: {
            double z = Math.rint(x);
            if (Math.abs(x - z) == 0.5) {
               return x + 1.0;
            } else {
               return z;
            }
         }
         case 2: {
            double z = Math.rint(x);
            if (Math.abs(x - z) == 0.5) {
               return x;
            } else {
               return z;
            }
         }
      }
      throw new IllegalStateException();
   }

   public static int nested(int kind, int[] data) {
      int total = 0;
      switch (kind) {
         case 0: {
            int length = data.length;
            for (int i = 0; i < length; i++) {
               total += data[i];
            }
            break;
         }
         case 1:
            if (data.length > 2) {
               int length = data[0] * 2;
               total = length + data[1];
            }
            break;
      }
      return total;
   }
}
