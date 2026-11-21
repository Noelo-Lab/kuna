package pkg;

public enum TestEnumNoConstants {
   ;

   private static int counter;

   public static int next() {
      return ++counter;
   }

   enum Empty {
   }

   public static class Holder {
      public int value = TestEnumNoConstants.next();
   }

   enum Only {
      ;

      static int twice(int x) {
         return x * 2;
      }
   }
}
