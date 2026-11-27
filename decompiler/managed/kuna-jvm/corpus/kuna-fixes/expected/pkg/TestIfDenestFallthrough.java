package pkg;

public class TestIfDenestFallthrough {
   private boolean closed;
   private int count;

   public void assertThenTryFinally(int x) {
      if (this.closed) {
         assert x > 0;

         try {
            this.run();
         } finally {
            this.cleanup();
         }
      }
   }

   public int tryFinallyThenCode(int x) {
      if (this.closed) {
         if (x <= 0) {
            return 1;
         }

         try {
            this.run();
         } finally {
            this.cleanup();
         }
      }

      this.count++;
      return 0;
   }

   public void tryCatchReturn(int x) {
      if (this.closed) {
         if (x <= 0) {
            return;
         }

         try {
            this.run();
         } catch (RuntimeException e) {
            return;
         }
      }

      this.count++;
   }

   public int tryFinallyThenReturn(int x) {
      if (this.closed) {
         if (x <= 0) {
            return 1;
         }

         try {
            this.run();
         } finally {
            this.cleanup();
         }
      }

      return 0;
   }

   private void run() {
   }

   private void cleanup() {
   }
}
