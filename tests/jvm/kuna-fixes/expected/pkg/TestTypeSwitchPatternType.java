package pkg;

public class TestTypeSwitchPatternType {
   static Integer next() {
      return 3;
   }

   static void sink(String var0) {
   }

   static void boxed(Integer var0) {
      switch (var0) {
         case 42:
            sink("42");
            break;
         case Integer var3 when var3 < 50:
            sink("small");
            break;
         default:
            sink("other");
      }
   }

   static void local() {
      Integer var0 = next();
      switch (var0) {
         case 42:
            sink("42");
            break;
         case Integer var3 when var3 < 50:
            sink("small");
            break;
         default:
            sink("other");
      }
   }

   static void string(String var0) {
      switch (var0) {
         case "a":
            sink("a");
            break;
         case String var3 when var3.length() > 3:
            sink("long");
            break;
         default:
            sink("other");
      }
   }

   static void character(Character var0) {
      switch (var0) {
         case 'a':
            sink("a");
            break;
         case Character var3 when var3 > 'x':
            sink("late");
            break;
         default:
            sink("other");
      }
   }
}
