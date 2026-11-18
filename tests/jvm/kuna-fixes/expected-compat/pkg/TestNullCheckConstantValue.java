package pkg;

public class TestNullCheckConstantValue {
   static int foo(int a, TestNullCheckConstantValue.Sub s) {
      return a + s.v();
   }

   int anonymous(TestNullCheckConstantValue.Box x) {
      return foo(5, new TestNullCheckConstantValue.Sub(x) {});
   }

   static int other(TestNullCheckConstantValue.Box b) {
      return 7;
   }

   static int constant(TestNullCheckConstantValue.Box b) {
      return 3;
   }

   static String name(TestNullCheckConstantValue.Box b) {
      return "box";
   }

   static int string(String s) {
      return 5;
   }

   static int object(Object o) {
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
