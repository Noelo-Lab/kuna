package pkg;

public class TestNullReceiverType {
   interface Reader {
      String next();
   }

   static class Cursor {
      int pos;

      String read() {
         return "x";
      }
   }

   static String take(Reader reader) {
      return reader.next();
   }

   static String receiver() {
      Cursor cursor = null;
      return cursor.read();
   }

   static String receiverAndArgument() {
      Reader reader = null;
      reader.next();
      return take(reader);
   }

   static Object array() {
      int[] values = null;
      return values.clone();
   }

   static int field() {
      Cursor cursor = null;
      return cursor.pos;
   }
}
