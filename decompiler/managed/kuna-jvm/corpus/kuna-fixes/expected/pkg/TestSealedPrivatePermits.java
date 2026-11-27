package pkg;

public abstract sealed class TestSealedPrivatePermits {
   public abstract int size();

   public static TestSealedPrivatePermits of(int n) {
      return n > 8 ? new TestSealedPrivatePermits.Large(n) : new TestSealedPrivatePermits.Small();
   }

   public record Circle(int r) implements TestSealedPrivatePermits.Shape {
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

   static sealed class Open permits TestSealedPrivatePermits.Open.Inner {
      private static final class Inner extends TestSealedPrivatePermits.Open {
      }
   }

   public sealed interface Shape permits TestSealedPrivatePermits.Circle, TestSealedPrivatePermits.Square {
   }

   private static final class Small extends TestSealedPrivatePermits {
      @Override
      public int size() {
         return 1;
      }
   }

   public record Square(int s) implements TestSealedPrivatePermits.Shape {
   }
}
