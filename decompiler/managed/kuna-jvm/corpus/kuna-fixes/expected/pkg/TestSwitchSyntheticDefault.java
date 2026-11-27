package pkg;

public class TestSwitchSyntheticDefault {
   static int sink;

   static long visit(TestSwitchSyntheticDefault.Kind k, long n) {
      return switch (k) {
         case ONE -> n * 2L;
      };
   }

   static long both(TestSwitchSyntheticDefault.Format f, long n) {
      return switch (f) {
         case FLAT -> n * 2L;
         case TREE -> throw new IllegalStateException("tree");
      };
   }

   static int area(TestSwitchSyntheticDefault.Shape s) {
      return switch (s) {
         case TestSwitchSyntheticDefault.Circle c -> c.r * c.r * 3;
         case TestSwitchSyntheticDefault.Square q -> q.s * q.s;
      };
   }

   static String name(TestSwitchSyntheticDefault.Shape s) {
      String n = switch (s) {
         case TestSwitchSyntheticDefault.Circle c -> "circle";
         case TestSwitchSyntheticDefault.Square q -> "square";
      };
      return n + "!";
   }

   static void describe(TestSwitchSyntheticDefault.Shape s) {
      switch (s) {
         case TestSwitchSyntheticDefault.Circle c:
            sink = c.r;
            break;
         case TestSwitchSyntheticDefault.Square q:
            sink = q.s;
            break;
      }
   }

   static final class Circle implements TestSwitchSyntheticDefault.Shape {
      int r;
   }

   enum Format {
      FLAT,
      TREE;
   }

   enum Kind {
      ONE;
   }

   sealed interface Shape permits TestSwitchSyntheticDefault.Circle, TestSwitchSyntheticDefault.Square {
   }

   static final class Square implements TestSwitchSyntheticDefault.Shape {
      int s;
   }
}
