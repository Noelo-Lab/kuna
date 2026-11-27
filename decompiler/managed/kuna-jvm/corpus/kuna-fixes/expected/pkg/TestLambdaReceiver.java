package pkg;

import java.util.function.Supplier;

public class TestLambdaReceiver {
   static int lambda() {
      return ((TestLambdaReceiver.Count)(() -> 3)).get();
   }

   static void reference(TestLambdaReceiver r) {
      ((Runnable)(r::run)).run();
   }

   static String generic() {
      String s = (String)((Supplier)(() -> "x")).get();
      return s;
   }

   void run() {
   }

   interface Count {
      int get();
   }
}
