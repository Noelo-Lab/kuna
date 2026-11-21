package pkg;

public class TestBoxedTypeVariableCast {
   static <T> T zero(Class<T> type, int kind) {
      if (kind == 0) {
         return (T)'\u0000';
      } else if (kind == 1) {
         return (T)0;
      } else {
         throw new IllegalArgumentException();
      }
   }

   static <N extends Number> N narrow(Number number) {
      return (N)number.byteValue();
   }

   static <N> N widen(Number number) {
      return (N)number.doubleValue();
   }

   static Object box(long value) {
      return value;
   }
}
