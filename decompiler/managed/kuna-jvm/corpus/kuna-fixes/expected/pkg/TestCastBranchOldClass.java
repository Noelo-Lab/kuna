package pkg;

public class TestCastBranchOldClass {
   Object last;
   int state;

   RuntimeException close(boolean compact) {
      RuntimeException he = null;

      try {
         if (compact) {
            this.reopen();
            this.state = 2;
         }
      } catch (Throwable t) {
         if (t instanceof IllegalStateException) {
            he = (IllegalStateException)t;
         } else {
            he = new IllegalStateException(t);
         }
      }

      this.state = 3;
      return he;
   }

   void reopen() {
      this.last = null;
   }
}
