package pkg;

public class TestTypeSwitchEnumLabel {
   static TestTypeSwitchEnumLabel.Shape next() {
      return TestTypeSwitchEnumLabel.Kind.CIRCLE;
   }

   static void sink(String s) {
   }

   void statement(TestTypeSwitchEnumLabel.Shape s) {
      switch (s) {
         case TestTypeSwitchEnumLabel.Kind.CIRCLE:
            sink("circle");
            break;
         case TestTypeSwitchEnumLabel.Kind k:
            sink("kind");
            break;
         case TestTypeSwitchEnumLabel.Polygon p:
            sink("polygon");
            break;
         default:
            sink("other");
      }
   }

   int expression(TestTypeSwitchEnumLabel.Shape s) {
      return switch (s) {
         case TestTypeSwitchEnumLabel.Kind.CIRCLE -> 1;
         case TestTypeSwitchEnumLabel.Kind.SQUARE -> 2;
         case TestTypeSwitchEnumLabel.Polygon p -> 3;
         default -> 4;
      };
   }

   void guardedLocal() {
      TestTypeSwitchEnumLabel.Shape s = next();
      switch (s) {
         case null:
            sink("null");
            break;
         case TestTypeSwitchEnumLabel.Kind.SQUARE:
            sink("square");
            break;
         case TestTypeSwitchEnumLabel.Kind k when k == TestTypeSwitchEnumLabel.Kind.CIRCLE:
            sink("circle");
            break;
         default:
            sink("other");
      }
   }

   enum Kind implements TestTypeSwitchEnumLabel.Shape {
      CIRCLE,
      SQUARE;
   }

   static final class Polygon implements TestTypeSwitchEnumLabel.Shape {
   }

   interface Shape {
   }
}
