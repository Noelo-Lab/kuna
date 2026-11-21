package pkg;

public class TestTypeSwitchEnumLabel {
   interface Shape {
   }

   enum Kind implements Shape {
      CIRCLE,
      SQUARE
   }

   static final class Polygon implements Shape {
   }

   static Shape next() {
      return Kind.CIRCLE;
   }

   static void sink(String s) {
   }

   void statement(Shape s) {
      switch (s) {
         case Kind.CIRCLE -> sink("circle");
         case Kind k -> sink("kind");
         case Polygon p -> sink("polygon");
         default -> sink("other");
      }
   }

   int expression(Shape s) {
      return switch (s) {
         case Kind.CIRCLE -> 1;
         case Kind.SQUARE -> 2;
         case Polygon p -> 3;
         default -> 4;
      };
   }

   void guardedLocal() {
      Shape s = next();
      switch (s) {
         case null -> sink("null");
         case Kind.SQUARE -> sink("square");
         case Kind k when k == Kind.CIRCLE -> sink("circle");
         default -> sink("other");
      }
   }
}
