package pkg;

public class TestTypeSwitchLabelLookup {
   static void sink(String s) {
   }

   void arrow(Object o) {
      switch (o) {
         case Integer _, String _:
            sink("number or text");
            break;
         default:
            sink("other");
      }
   }

   void guarded(Object o, boolean b) {
      switch (o) {
         case Integer _, String _ when b:
            sink("number or text");
            break;
         default:
            sink("other");
      }
   }

   int expression(Object o) {
      return switch (o) {
         case Integer _, String _ -> 1;
         case Long l -> l.intValue();
         default -> 3;
      };
   }

   void shapes(TestTypeSwitchLabelLookup.Shape s) {
      switch (s) {
         case TestTypeSwitchLabelLookup.Kind.CIRCLE:
         case TestTypeSwitchLabelLookup.Kind.SQUARE:
            sink("kind");
            break;
         case TestTypeSwitchLabelLookup.Polygon _, TestTypeSwitchLabelLookup.Line _, TestTypeSwitchLabelLookup.Point _:
            sink("figure");
            break;
         default:
            sink("other");
      }
   }

   void constants(Integer o) {
      switch (o) {
         case 5:
         case 7:
            sink("5 or 7");
            break;
         case 1:
            sink("1");
            break;
         case Integer i when i > 100:
            sink("big");
            break;
         default:
            sink("other");
      }
   }

   String substituted(Integer o) {
      return switch (o) {
         case 1 -> "one";
         case Integer i when i > 10 -> "big";
         default -> "other";
      };
   }

   enum Kind implements TestTypeSwitchLabelLookup.Shape {
      CIRCLE,
      SQUARE;
   }

   static final class Line implements TestTypeSwitchLabelLookup.Shape {
   }

   record Point(int x) implements TestTypeSwitchLabelLookup.Shape {
   }

   static final class Polygon implements TestTypeSwitchLabelLookup.Shape {
   }

   interface Shape {
   }
}
