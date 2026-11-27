package pkg;

public class TestNestedAlreadyCaught {
   static class BaseFailure extends RuntimeException {
   }

   static class SubFailure extends BaseFailure {
   }

   static void run() {
   }

   public void sameType(int x) {
      try {
         try {
            run();
            return;
         } catch (BaseFailure e) {
         }
      } catch (BaseFailure e2) {
         System.out.println(x);
      }
   }

   public void subType(int x) {
      try {
         try {
            run();
            return;
         } catch (BaseFailure e) {
         }
      } catch (SubFailure e2) {
         System.out.println(x);
      }
   }

   public void siblings(int x) {
      try {
         run();
         return;
      } catch (SubFailure e) {
      } catch (BaseFailure e2) {
         System.out.println(x);
      }
   }
}
