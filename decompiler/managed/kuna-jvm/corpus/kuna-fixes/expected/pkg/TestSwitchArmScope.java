package pkg;

public class TestSwitchArmScope {
   static int next() {
      return 4;
   }

   static int freed(int b) {
      int t = next();
      System.out.println(t);

      int num = switch (b) {
         case 0 -> {
            int u = next();
            yield u * 2;
         }
         default -> 7;
      };
      return num;
   }

   static int last(int b) {
      int num = switch (b) {
         case 0 -> {
            int k = 5;
            yield k;
         }
         default -> num = 8;
      };
      return num;
   }
}
