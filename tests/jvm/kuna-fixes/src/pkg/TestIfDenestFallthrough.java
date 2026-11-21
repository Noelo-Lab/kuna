package pkg;

public class TestIfDenestFallthrough {
   private boolean closed;
   private int count;

   public void assertThenTryFinally(int x) {
      if (closed) {
         assert x > 0;
         try {
            run();
         } finally {
            cleanup();
         }
      }
   }

   public int tryFinallyThenCode(int x) {
      if (closed) {
         if (x <= 0) {
            return 1;
         }
         try {
            run();
         } finally {
            cleanup();
         }
      }
      count++;
      return 0;
   }

   public void tryCatchReturn(int x) {
      if (closed) {
         if (x <= 0) {
            return;
         }
         try {
            run();
         } catch (RuntimeException e) {
            return;
         }
      }
      count++;
   }

   public int tryFinallyThenReturn(int x) {
      if (closed) {
         if (x <= 0) {
            return 1;
         }
         try {
            run();
         } finally {
            cleanup();
         }
      }
      return 0;
   }

   private void run() {
   }

   private void cleanup() {
   }
}
