package pkg;

public enum TestEnumNoConstants {
   ;

   private static int counter;

   public static int next() {
      return ++counter;
   }

   public static class Holder {
      public int value = next();
   }

   enum Empty {
   }

   enum Only {
      ;

      static int twice(int x) {
         return x * 2;
      }
   }
}
