package pkg;

public class TestBoxedTypeVariableCast {
   static <T> T zero(Class<T> type, int kind) {
      if (kind == 0) {
         return (T)Character.valueOf('\u0000');
      }

      if (kind == 1) {
         return (T)Integer.valueOf(0);
      }

      throw new IllegalArgumentException();
   }

   static <N extends Number> N narrow(Number number) {
      return (N)Byte.valueOf(number.byteValue());
   }

   static <N> N widen(Number number) {
      return (N)Double.valueOf(number.doubleValue());
   }

   static Object box(long value) {
      return value;
   }
}
