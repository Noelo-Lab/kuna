package pkg;

public class TestEcjStringSwitch {
   static TestEcjStringSwitch.Kind kind(String word) {
      String var1 = word;
      switch (word.hashCode()) {
         case 112785:
            if (var1.equals("red")) {
               return TestEcjStringSwitch.Kind.RED;
            }
            break;
         case 3027034:
            if (var1.equals("blue")) {
               return TestEcjStringSwitch.Kind.BLUE;
            }
            break;
         case 98619139:
            if (var1.equals("green")) {
               return TestEcjStringSwitch.Kind.GREEN;
            }
      }

      return TestEcjStringSwitch.Kind.NONE;
   }

   enum Kind {
      RED,
      GREEN,
      BLUE,
      NONE;
   }
}
