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
      try (this.current) {
         this.work();
      }
   }

   public void logsCloseFailure() {
      try (TestTwrDesugaringShape.Res r = this.open()) {
         this.work();
      }
   }

   public void returnSkipsClose(boolean b) {
      try (TestTwrDesugaringShape.Res r = this.open()) {
         if (b) {
            return;
         }

         this.work();
      }
   }

   public void workBeforeTry() {
      this.work();

      try (TestTwrDesugaringShape.Res r = this.open()) {
         this.work();
      }
   }

   public void rethrowsWrapped() {
      try (TestTwrDesugaringShape.Res r = this.open()) {
         this.work();
      }
   }

   public void closesUnchecked() {
      try (TestTwrDesugaringShape.Res r = this.open()) {
         this.work();
      }
   }

   void log(Throwable t) {
      this.trace = this.trace + "log;";
   }

   interface Res extends AutoCloseable {
      void close();

      int read();
   }
}
