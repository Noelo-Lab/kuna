package pkg;

public class TestSwitchTableFillers {
   static int width(int len) {
      switch (len) {
         case 1:
            return 8;
         case 2:
            return 16;
         case 3:
         default:
            throw new IllegalArgumentException("len " + len);
         case 4:
            return 32;
      }
   }

   static int explicitGap(int x) {
      switch (x) {
         case 1:
            return 10;
         case 2:
         default:
            return 99;
         case 3:
            return 30;
      }
   }

   static long size(int n) {
      long v = 0L;
      switch (n) {
         case 1:
            v = 1L;
            break;
         case 2:
            v = 2L;
         case 3:
         case 5:
         case 6:
         case 7:
         default:
            break;
         case 4:
            v = 4L;
            break;
         case 8:
            v = 8L;
      }

      return v;
   }

   static int groups(int x) {
      switch (x) {
         case 1:
         case 5:
            return 2;
         case 2:
         case 8:
            return 1;
         case 3:
         case 4:
         case 6:
         case 7:
         default:
            return 0;
      }
   }
}
