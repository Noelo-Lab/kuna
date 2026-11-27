package pkg;

public class TestNullLocalType {
   interface Source {
      String read();
   }

   static String take(Source source) {
      return source == null ? "none" : source.read();
   }

   static void take(Object other) {
   }

   static int length(int[] values) {
      return values == null ? 0 : values.length;
   }

   static String overload() {
      Source source = null;
      return take(source);
   }

   static int array() {
      int[] values = null;
      return length(values);
   }

   Source field;

   void store() {
      Source source = null;
      this.field = source;
   }
}
