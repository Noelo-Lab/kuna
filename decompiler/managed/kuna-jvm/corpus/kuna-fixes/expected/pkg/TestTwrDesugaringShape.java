package pkg;

public class TestTwrDesugaringShape {
   private final TestTwrDesugaringShape.Res fixed;
   private TestTwrDesugaringShape.Res current;
   int count;
   boolean failing;
   String trace = "";

   TestTwrDesugaringShape(TestTwrDesugaringShape.Res fixed) {
      this.fixed = fixed;
      this.current = fixed;
   }

   TestTwrDesugaringShape.Res open() {
      this.trace = this.trace + "open;";
      return this.current;
   }

   void work() {
      this.trace = this.trace + "work;";
      if (this.failing) {
         throw new RuntimeException("work");
      }

      this.count++;
   }

   public int real() {
      try (TestTwrDesugaringShape.Res r = this.open()) {
         return r.read();
      }
   }

   public void finalField() {
      try (this.fixed) {
         this.count = this.fixed.read();
      }
   }

   public void plainField() {
      try (TestTwrDesugaringShape.Res r = this.current) {
         this.work();
      }
   }

   public void logsCloseFailure() {
      TestTwrDesugaringShape.Res r = this.open();

      try {
         this.work();
      } catch (Throwable t) {
         if (r != null) {
            try {
               r.close();
            } catch (Throwable t2) {
               this.log(t2);
            }
         }

         throw t;
      }

      if (r != null) {
         r.close();
      }
   }

   public void returnSkipsClose(boolean b) {
      TestTwrDesugaringShape.Res r = this.open();

      try {
         if (b) {
            return;
         }

         this.work();
      } catch (Throwable t) {
         if (r != null) {
            try {
               r.close();
            } catch (Throwable t2) {
               t.addSuppressed(t2);
            }
         }

         throw t;
      }

      if (r != null) {
         r.close();
      }
   }

   public void workBeforeTry() {
      TestTwrDesugaringShape.Res r = this.open();
      this.work();

      try {
         this.work();
      } catch (Throwable t) {
         if (r != null) {
            try {
               r.close();
            } catch (Throwable t2) {
               t.addSuppressed(t2);
            }
         }

         throw t;
      }

      if (r != null) {
         r.close();
      }
   }

   public void rethrowsWrapped() {
      TestTwrDesugaringShape.Res r = this.open();

      try {
         this.work();
      } catch (Throwable t) {
         if (r != null) {
            try {
               r.close();
            } catch (Throwable t2) {
               t.addSuppressed(t2);
            }
         }

         throw new IllegalStateException(t);
      }

      if (r != null) {
         r.close();
      }
   }

   public void closesUnchecked() {
      TestTwrDesugaringShape.Res r = this.open();

      try {
         this.work();
      } catch (Throwable t) {
         try {
            r.close();
         } catch (Throwable t2) {
            t.addSuppressed(t2);
         }

         throw t;
      }

      r.close();
   }

   void log(Throwable t) {
      this.trace = this.trace + "log;";
   }

   interface Res extends AutoCloseable {
      void close();

      int read();
   }
}
