package pkg;

import java.util.Objects;

public class TestSourceNullCheck {
   final int size = 5;

   static Holder wrap(Object o) {
      Objects.requireNonNull(o);
      return new Holder(o);
   }

   Inner innerOf(TestSourceNullCheck other) {
      Objects.requireNonNull(other);
      return new Inner(2);
   }

   static Object checked(Object o) {
      Objects.requireNonNull(o);
      return null;
   }

   static Class<?> classOf(Object o) {
      o.getClass();
      return null;
   }

   static int sizeOf(TestSourceNullCheck t) {
      return t.size;
   }

   static char[] hex(byte[] bytes) {
      Objects.requireNonNull(bytes);
      char[] result = new char[2 * bytes.length];
      return result;
   }

   static Inner qualified(TestSourceNullCheck t) {
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
