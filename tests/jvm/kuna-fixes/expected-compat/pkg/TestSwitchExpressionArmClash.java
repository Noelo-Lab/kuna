package pkg;

public class TestSwitchExpressionArmClash {
   static int add(String s) {
      return s.length();
   }

   public static int typeOf(String buffer, int offset) {
      switch (buffer.charAt(offset)) {
         case 'L':
            String internalName = buffer.substring(offset + 1, buffer.length() - 1);
            return 7 | add(internalName);
         case 'V':
            return 0;
         case '[':
            int element = offset + 1;

            while (buffer.charAt(element) == '[') {
               element++;
            }
            return element - offset << 26 | switch (buffer.charAt(element)) {
               case 'I' -> 1;
               case 'L' -> {
                  String internalName = buffer.substring(element + 1, buffer.length() - 1);
                  yield 8 | add(internalName);
               }
               default -> throw new IllegalArgumentException();
            };
         default:
            throw new IllegalArgumentException();
      }
   }
}
