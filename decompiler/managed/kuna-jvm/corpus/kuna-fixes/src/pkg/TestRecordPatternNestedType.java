package pkg;

public class TestRecordPatternNestedType {
   sealed interface Shape permits Box, Label {
   }

   record Box(Object content) implements Shape {
   }

   record Label(String text) implements Shape {
   }

   record Pair(Object first, Object second) {
   }

   static String describe(Shape shape) {
      return switch (shape) {
         case Box(String s) -> s;
         case Label(String t) -> t;
         default -> throw new IllegalStateException();
      };
   }

   static String firstText(Object o) {
      if (o instanceof Pair(Number n, String s)) {
         return s + n.intValue();
      }
      return null;
   }
}
