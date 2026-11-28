package pkg;

public class TestFinallyBodyThrows {
   public void throwsFromFinally(RuntimeException t) {
      try {
         System.out.println(1);
      } catch (Throwable e) {
         e.printStackTrace();
      } finally {
         throw t;
      }
   }

   public int returnsFromFinally(int x) {
      try {
         x = 10 / x;
         return x;
      } finally {
         return x;
      }
   }
}
