package pkg;

public class TestSwitchExprPolyCast {
   public Number intOrLong(int x) {
      return switch (x) {
         case 1 -> 1;
         default -> 2L;
      };
   }

   public Comparable<?> charOrInt(int x) {
      return switch (x) {
         case 1 -> 'a';
         default -> 1;
      };
   }

   public void argument(int x) {
      take(switch (x) {
         case 1 -> 1;
         case 2 -> 2.5;
         default -> 3L;
      });
   }

   public Object toObject(int x) {
      return switch (x) {
         case 1 -> 1;
         default -> 2L;
      };
   }

   public Number sameType(int x) {
      return switch (x) {
         case 1 -> 1;
         default -> 2;
      };
   }

   static void take(Number n) {
   }
}
