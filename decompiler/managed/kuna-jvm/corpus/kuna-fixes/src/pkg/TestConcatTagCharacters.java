// A concatenation constant holding the recipe's tag characters is written: javac -g --release 17
package pkg;

public class TestConcatTagCharacters {
   static final char SEPARATOR = '\0';

   static String upper(String key) {
      return key + (char) (SEPARATOR + 1);
   }

   static String tagInside(String key, int n) {
      return key + "a\u0001b" + n;
   }

   static String constantTag(String key) {
      return key + "\u0002";
   }

   static String both(String key) {
      return "\u0001" + key + "\u0002";
   }
}
