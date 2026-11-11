package pkg;

public class TestShadowedTypeVariable<T extends Number> {
   T value;

   static int use(Number n) {
      return 1;
   }

   static int use(Object o) {
      return 2;
   }

   <T> int method(T t) {
      return use((Number)t);
   }

   class Inner {
      <T> int pick(T t) {
         return TestShadowedTypeVariable.use(t);
      }

      <T> Number widen(T t) {
         return t;
      }
   }
}
