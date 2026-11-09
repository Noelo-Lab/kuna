package pkg;

public class TestTypeSwitchPatternType {
   static Integer next() {
      return 3;
   }

   static void sink(String s) {
   }

   static void boxed(Integer o) {
      switch (o) {
         case 42 -> sink("42");
         case Integer i when i < 50 -> sink("small");
         default -> sink("other");
      }
   }

   static void local() {
      Integer o = next();
      switch (o) {
         case 42 -> sink("42");
         case Integer i when i < 50 -> sink("small");
         default -> sink("other");
      }
   }

   static void string(String s) {
      switch (s) {
         case "a" -> sink("a");
         case String t when t.length() > 3 -> sink("long");
         default -> sink("other");
      }
   }

   static void character(Character c) {
      switch (c) {
         case 'a' -> sink("a");
         case Character d when d > 'x' -> sink("late");
         default -> sink("other");
      }
   }
}
