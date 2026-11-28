// A loop's update that ends a switch's last case, reached by falling into it: javac -g --release 17
package pkg;

public class TestUpdateInLastCase {
   static byte[] discardWhitespace(byte[] data) {
      byte[] groomed = new byte[data.length];
      int copied = 0;
      for (int i = 0; i < data.length; i++) {
         switch (data[i]) {
            case ' ':
            case '\n':
               break;
            default:
               groomed[copied++] = data[i];
         }
      }
      return groomed;
   }
}
