package pkg;

public class TestSourceNullCheck {
   final int size = 5;

   static TestSourceNullCheck.Holder wrap(Object o) {
      return new TestSourceNullCheck.Holder(o);
   }

   TestSourceNullCheck.Inner innerOf(TestSourceNullCheck other) {
      return new TestSourceNullCheck.Inner(2);
   }

   static Object checked(Object o) {
      return null;
   }

   static Class<?> classOf(Object o) {
      return null;
   }

   static int sizeOf(TestSourceNullCheck t) {
      return 5;
   }

   static char[] hex(byte[] bytes) {
      return new char[2 * bytes.length];
   }

   static TestSourceNullCheck.Inner qualified(TestSourceNullCheck t) {
      return t.new Inner(7);
   }

   static class Holder {
      final Object o;

      Holder(Object o) {
         this.o = o;
      }
   }

   class Inner {
      final int k;

      Inner(int k) {
         this.k = k;
      }
   }
}
