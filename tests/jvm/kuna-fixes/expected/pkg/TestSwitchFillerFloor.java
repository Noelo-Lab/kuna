package pkg;

public class TestSwitchFillerFloor {
   static int width(int kind, int[] out) {
      int size = 0;
      switch (kind) {
         case 16:
         case 17:
         case 18:
            size = 1;
            break;
         case 64:
         case 65:
         case 66:
         case 67:
         case 68:
         case 69:
         case 70:
            out[0] = kind;
            size = 2;
            break;
         case 71:
         case 72:
         case 73:
         case 74:
         case 75:
            out[1] = kind;
            size = 4;
            break;
         case 0:
         case 1:
         case 2:
         default:
            out[2] = kind;
      }

      return size;
   }
}
