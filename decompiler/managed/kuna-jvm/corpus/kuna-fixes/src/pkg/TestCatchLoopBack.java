package pkg;

public class TestCatchLoopBack {
   private Source in;
   private int attempt;
   private long offset;

   public int read() throws IoError {
      int initial = this.attempt;
      while (true) {
         try {
            int result = this.in.read();
            if (result != -1) {
               this.offset++;
            }
            this.record(initial);
            return result;
         } catch (IoError e) {
            this.retry(initial, e);
         } catch (StateError e) {
            this.retry(initial, e);
         }
      }
   }

   public int readThree() throws IoError {
      this.record(0);
      while (true) {
         try {
            return this.in.read();
         } catch (IoError e) {
            this.record(1);
         } catch (StateError e) {
            this.record(2);
         } catch (OtherError e) {
            this.record(3);
         }
      }
   }

   public int readOrDefault() throws IoError {
      this.record(0);
      while (true) {
         try {
            return this.in.read();
         } catch (IoError e) {
            this.record(1);
         } catch (StateError e) {
            this.record(2);
         } catch (OtherError e) {
            break;
         }
      }
      return -1;
   }

   public int readGuarded(boolean log) throws IoError {
      if (log) {
         this.record(0);
      }
      try {
         while (true) {
            try {
               return this.in.read();
            } catch (IoError e) {
               this.record(1);
            } catch (StateError e) {
               this.record(2);
            }
         }
      } catch (OtherError e) {
         return 0;
      }
   }

   public int readFinally() throws IoError {
      this.record(0);
      try {
         while (true) {
            try {
               return this.in.read();
            } catch (IoError e) {
               this.record(1);
            } catch (StateError e) {
               this.record(2);
            }
         }
      } finally {
         this.record(5);
      }
   }

   public int readOneCatch() throws IoError {
      this.record(0);
      while (true) {
         try {
            return this.in.read();
         } catch (IoError e) {
            this.record(1);
         }
      }
   }

   void record(int a) {
   }

   void retry(int a, Exception e) {
   }

   interface Source {
      int read() throws IoError;
   }

   static class IoError extends Exception {
   }

   static class StateError extends RuntimeException {
   }

   static class OtherError extends RuntimeException {
   }
}
