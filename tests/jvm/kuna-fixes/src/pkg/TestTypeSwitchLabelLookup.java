package pkg;

public class TestTypeSwitchLabelLookup {
   interface Shape {
   }

   enum Kind implements Shape {
      CIRCLE,
      SQUARE
   }

   static final class Polygon implements Shape {
   }

   static final class Line implements Shape {
   }

   record Point(int x) implements Shape {
   }

   static void sink(String s) {
   }

   void arrow(Object o) {
      switch (o) {
         case Integer _, String _ -> sink("number or text");
         default -> sink("other");
      }
   }

   void guarded(Object o, boolean b) {
      switch (o) {
         case Integer _, String _ when b -> sink("number or text");
         default -> sink("other");
      }
   }

   int expression(Object o) {
      return switch (o) {
         case Integer _, String _ -> 1;
         case Long l -> l.intValue();
         default -> 3;
      };
   }

   void shapes(Shape s) {
      switch (s) {
         case Kind.CIRCLE, Kind.SQUARE -> sink("kind");
         case Polygon _, Line _, Point _ -> sink("figure");
         default -> sink("other");
      }
   }

   void constants(Integer o) {
      switch (o) {
         case 5, 7 -> sink("5 or 7");
         case 1 -> sink("1");
         case Integer i when i > 100 -> sink("big");
         default -> sink("other");
      }
   }

   String substituted(Integer o) {
      return switch (o) {
         case 1 -> "one";
         case Integer i when i > 10 -> "big";
         default -> "other";
      };
   }
}
