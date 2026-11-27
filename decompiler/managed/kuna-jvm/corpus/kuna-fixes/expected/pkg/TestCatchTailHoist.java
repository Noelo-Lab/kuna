package pkg;

public class TestCatchTailHoist {
   int logged;

   int invoke(TestCatchTailHoist.Source s, boolean enabled) {
      if (enabled) {
         try {
            return s.get();
         } catch (TestCatchTailHoist.Problem ex) {
            if (!this.expected(ex)) {
               throw ex;
            }

            this.log(ex);
         }
      }

      return -1;
   }

   int first(TestCatchTailHoist.Source[] sources) {
      for (TestCatchTailHoist.Source s : sources) {
         try {
            return s.get();
         } catch (TestCatchTailHoist.Problem ex) {
            if (!this.expected(ex)) {
               throw ex;
            }

            this.log(ex);
         }
      }

      return -1;
   }

   int retry(TestCatchTailHoist.Source s, int tries) {
      while (tries-- > 0) {
         try {
            return s.get();
         } catch (TestCatchTailHoist.Problem ex) {
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

   boolean expected(TestCatchTailHoist.Problem ex) {
      return ex.getMessage() == null;
   }

   void log(TestCatchTailHoist.Problem ex) {
      this.logged++;
   }

   static class Problem extends RuntimeException {
   }

   interface Source {
      int get();
   }
}
