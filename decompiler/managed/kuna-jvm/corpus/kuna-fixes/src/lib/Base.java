package lib;

public abstract class Base {
   protected static class Helper {
      public final Object value;

      public Helper(Object value) {
         this.value = value;
      }
   }

   protected static class Holder {
      public static class Deep {
         public Deep() {
         }
      }
   }

   protected static class Marker {
      public Marker() {
      }
   }

   protected enum Status {
      YES,
      NO,
      END;
   }

   public abstract Object make(Object o);
}
