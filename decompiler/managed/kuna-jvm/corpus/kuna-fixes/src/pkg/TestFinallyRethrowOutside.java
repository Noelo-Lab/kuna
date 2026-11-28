package pkg;

public class TestFinallyRethrowOutside {
   static void work() {
   }

   public int breakInFinally(int a) {
      while (true) {
         try {
            work();
         } catch (RuntimeException e) {
            a--;
         } finally {
            if (a > 3) {
               break;
            }
         }
         a += 2;
      }
      return a;
   }
}
