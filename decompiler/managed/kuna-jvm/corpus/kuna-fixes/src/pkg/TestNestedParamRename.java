package pkg;

public class TestNestedParamRename {
   interface Handler {
      void onFailure(Exception e);
   }

   static void run(Handler h) {
   }

   static void log(Exception e, Object target) {
   }

   static Object lookup(String action) throws Exception {
      return action;
   }

   public static void send(String action) {
      try {
         final Object reg = lookup(action);
         run(new Handler() {
            @Override
            public void onFailure(Exception e) {
               log(e, reg);
            }
         });
      } catch (Exception e) {
         log(e, null);
      }
   }
}
