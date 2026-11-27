package pkg;

public class TestCatchLoopBack {
   private TestCatchLoopBack.Source in;
   private int attempt;
   private long offset;

   public int read() throws TestCatchLoopBack.IoError {
      int initial = this.attempt;

      while (true) {
         try {
            int result = this.in.read();
            if (result != -1) {
               this.offset++;
            }

            this.record(initial);
            return result;
         } catch (TestCatchLoopBack.IoError e) {
            this.retry(initial, e);
         } catch (TestCatchLoopBack.StateError e) {
            this.retry(initial, e);
         }
      }
   }

   public int readThree() throws TestCatchLoopBack.IoError {
      this.record(0);

      while (true) {
         try {
            return this.in.read();
         } catch (TestCatchLoopBack.IoError e) {
            this.record(1);
         } catch (TestCatchLoopBack.StateError e) {
            this.record(2);
         } catch (TestCatchLoopBack.OtherError e) {
            this.record(3);
         }
      }
   }

   public int readOrDefault() throws TestCatchLoopBack.IoError {
      this.record(0);

      while (true) {
         try {
            return this.in.read();
         } catch (TestCatchLoopBack.IoError e) {
            this.record(1);
         } catch (TestCatchLoopBack.StateError e) {
            this.record(2);
         } catch (TestCatchLoopBack.OtherError e) {
            return -1;
         }
      }
   }

   public int readGuarded(boolean log) throws TestCatchLoopBack.IoError {
      if (log) {
         this.record(0);
      }

      try {
         while (true) {
            try {
               return this.in.read();
            } catch (TestCatchLoopBack.IoError e) {
               this.record(1);
            } catch (TestCatchLoopBack.StateError e) {
               this.record(2);
            }
         }
      } catch (TestCatchLoopBack.OtherError e) {
         return 0;
      }
   }

   public int readFinally() throws TestCatchLoopBack.IoError {
      this.record(0);

      try {
         while (true) {
            try {
               return this.in.read();
            } catch (TestCatchLoopBack.IoError e) {
               this.record(1);
            } catch (TestCatchLoopBack.StateError e) {
               this.record(2);
            }
         }
      } finally {
         this.record(5);
      }
   }

   public int readOneCatch() throws TestCatchLoopBack.IoError {
      this.record(0);

      while (true) {
         try {
            return this.in.read();
         } catch (TestCatchLoopBack.IoError e) {
            this.record(1);
         }
      }
   }

   void record(int a) {
   }

   void retry(int a, Exception e) {
   }

   static class IoError extends Exception {
   }

   static class OtherError extends RuntimeException {
   }

   interface Source {
      int read() throws TestCatchLoopBack.IoError;
   }

   static class StateError extends RuntimeException {
   }
}
