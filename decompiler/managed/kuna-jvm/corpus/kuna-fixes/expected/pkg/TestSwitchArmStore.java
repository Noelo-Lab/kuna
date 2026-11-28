package pkg;

public class TestSwitchArmStore {
   int plain(int bar) {
      int num = switch (bar) {
         case 0 -> num = 1;
         case 1 -> {
            int t = bar + 5;
            yield t + 1;
         }
         default -> 0;
      };
      return num;
   }

   String text(int k) {
      String s = switch (k) {
         case 0 -> s = "a";
         case 1 -> {
            s = "b" + k;
            yield s + s;
         }
         default -> "c";
      };
      return s;
   }

   int reassigned(int k) {
      int x = k * 2;

      x = switch (k) {
         case 1 -> x = 4;
         case 2 -> x + 1;
         default -> -x;
      };
      return x;
   }
}
