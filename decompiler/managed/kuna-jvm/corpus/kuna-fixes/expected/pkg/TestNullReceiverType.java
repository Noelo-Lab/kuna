package pkg;

public class TestNullReceiverType {
   static String take(TestNullReceiverType.Reader var0) {
      return var0.next();
   }

   static String receiver() {
      TestNullReceiverType.Cursor var0 = null;
      return var0.read();
   }

   static String receiverAndArgument() {
      TestNullReceiverType.Reader var0 = null;
      var0.next();
      return take(var0);
   }

   static Object array() {
      int[] var0 = null;
      return var0.clone();
   }

   static int field() {
      TestNullReceiverType.Cursor var0 = null;
      return var0.pos;
   }

   static class Cursor {
      int pos;

      String read() {
         return "x";
      }
   }

   interface Reader {
      String next();
   }
}
