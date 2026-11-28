package pkg;

public class TestRecordPatternNestedType {
   static String describe(TestRecordPatternNestedType.Shape var0) {
      return switch (var0) {
         case TestRecordPatternNestedType.Box(String var4) -> var4;
         case TestRecordPatternNestedType.Label(String var6) -> var6;
         default -> throw new IllegalStateException();
      };
   }

   static String firstText(Object var0) {
      if (var0 instanceof TestRecordPatternNestedType.Pair(Number var2, String var3)) {
         return var3 + var2.intValue();
      }

      return null;
   }

   record Box(Object content) implements TestRecordPatternNestedType.Shape {
   }

   record Label(String text) implements TestRecordPatternNestedType.Shape {
   }

   record Pair(Object first, Object second) {
   }

   sealed interface Shape permits TestRecordPatternNestedType.Box, TestRecordPatternNestedType.Label {
   }
}
