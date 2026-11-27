package pkg;

public abstract sealed class TestSealedPrivatePermits {
   public abstract int size();

   public static TestSealedPrivatePermits of(int n) {
      return n > 8 ? new Large(n) : new Small();
   }

   private static final class Small extends TestSealedPrivatePermits {
      @Override
      public int size() {
         return 1;
      }
   }

   private static non-sealed class Large extends TestSealedPrivatePermits {
      private final int n;

      Large(int n) {
         this.n = n;
      }

      @Override
      public int size() {
         return this.n;
      }
   }

   static sealed class Open permits Open.Inner {
      private static final class Inner extends Open {
      }
   }

   public sealed interface Shape permits Circle, Square {
   }

   public record Circle(int r) implements Shape {
   }

   public record Square(int s) implements Shape {
   }
}
