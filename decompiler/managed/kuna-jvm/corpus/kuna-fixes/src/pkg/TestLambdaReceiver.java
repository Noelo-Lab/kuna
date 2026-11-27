package pkg;

import java.util.function.Supplier;

public class TestLambdaReceiver {
   interface Count {
      int get();
   }

   static int lambda() {
      return ((Count) () -> 3).get();
   }

   static void reference(TestLambdaReceiver r) {
      ((Runnable) r::run).run();
   }

   static String generic() {
      String s = ((Supplier<String>) () -> "x").get();
      return s;
   }

   void run() {
   }
}
