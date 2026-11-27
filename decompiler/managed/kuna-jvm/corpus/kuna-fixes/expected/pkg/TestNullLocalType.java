package pkg;

public class TestNullLocalType {
   TestNullLocalType.Source field;

   static String take(TestNullLocalType.Source var0) {
      return var0 == null ? "none" : var0.read();
   }

   static void take(Object var0) {
   }

   static int length(int[] var0) {
      return var0 == null ? 0 : var0.length;
   }

   static String overload() {
      TestNullLocalType.Source var0 = null;
      return take(var0);
   }

   static int array() {
      int[] var0 = null;
      return length(var0);
   }

   static String cast() {
      Object var0 = null;
      return take((TestNullLocalType.Source)var0);
   }

   void store() {
      Object var1 = null;
      this.field = (TestNullLocalType.Source)var1;
   }

   interface Source {
      String read();
   }
}
