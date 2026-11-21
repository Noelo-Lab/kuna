package pkg;

public class TestCatchTailHoist {
   static class Problem extends RuntimeException {
   }

   interface Source {
      int get();
   }

   int logged;

   int invoke(Source s, boolean enabled) {
      if (enabled) {
         try {
            return s.get();
         } catch (Problem ex) {
            if (!this.expected(ex)) {
               throw ex;
            }
            this.log(ex);
         }
      }
      return -1;
   }

   int first(Source[] sources) {
      for (Source s : sources) {
         try {
            return s.get();
         } catch (Problem ex) {
            if (!this.expected(ex)) {
               throw ex;
            }
            this.log(ex);
         }
      }
      return -1;
   }

   int retry(Source s, int tries) {
      while (tries-- > 0) {
         try {
            return s.get();
         } catch (Problem ex) {
            if (tries == 0) {
               throw ex;
            }
            this.log(ex);
            if (this.logged > 5) {
               break;
            }
         }
      }
      return -2;
   }

   boolean expected(Problem ex) {
      return ex.getMessage() == null;
   }

   void log(Problem ex) {
      this.logged++;
   }
}
