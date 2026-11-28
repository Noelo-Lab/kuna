package pkg;

public class TestNestedAlreadyCaught {
   static void run() {
   }

   public void sameType(int x) {
      try {
         try {
            run();
            return;
         } catch (TestNestedAlreadyCaught.BaseFailure var3) {
         }
      } catch (TestNestedAlreadyCaught.BaseFailure e2) {
         System.out.println(x);
      }
   }

   public void subType(int x) {
      try {
         try {
            run();
            return;
         } catch (TestNestedAlreadyCaught.BaseFailure var3) {
         }
      } catch (TestNestedAlreadyCaught.SubFailure e2) {
         System.out.println(x);
      }
   }

   public void siblings(int x) {
      try {
         run();
         return;
      } catch (TestNestedAlreadyCaught.SubFailure var3) {
      } catch (TestNestedAlreadyCaught.BaseFailure e2) {
         System.out.println(x);
      }
   }

   static class BaseFailure extends RuntimeException {
   }

   static class SubFailure extends TestNestedAlreadyCaught.BaseFailure {
   }
}
