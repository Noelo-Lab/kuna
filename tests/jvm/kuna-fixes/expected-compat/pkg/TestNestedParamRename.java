package pkg;

public class TestNestedParamRename {
   static void run(TestNestedParamRename.Handler h) {
   }

   static void log(Exception e, Object target) {
   }

   static Object lookup(String action) throws Exception {
      return action;
   }

   public static void send(String action) {
      try {
         final Object reg = lookup(action);
         run(new TestNestedParamRename.Handler() {
            @Override
            public void onFailure(Exception ex) {
               TestNestedParamRename.log(e, reg);
            }
         });
      } catch (Exception e) {
         log(e, null);
      }
   }

   interface Handler {
      void onFailure(Exception var1);
   }
}
