package pkg;

import java.util.Objects;

public class TestNullCheckConstantValue {
   static int foo(int a, TestNullCheckConstantValue.Sub s) {
      return a + s.v();
   }

   int anonymous(TestNullCheckConstantValue.Box x) {
      Objects.requireNonNull(x);
      return foo(5, new TestNullCheckConstantValue.Sub(x) {});
   }

   static int other(TestNullCheckConstantValue.Box b) {
      Objects.requireNonNull(b);
      return 7;
   }

   static int constant(TestNullCheckConstantValue.Box b) {
      return b.k;
   }

   static String name(TestNullCheckConstantValue.Box b) {
      return b.name;
   }

   static int string(String s) {
      Objects.requireNonNull(s);
      return 5;
   }

   static int object(Object o) {
      o.getClass();
      return 9;
   }

   static class Box {
      final int k = 3;
      final String name = "box";
   }

   class Sub {
      Sub(TestNullCheckConstantValue.Box b) {
      }

      int v() {
         return 1;
      }
   }
}
