package pkg;

public class TestUpdateInLastCase {
   static byte[] discardWhitespace(byte[] data) {
      byte[] groomed = new byte[data.length];
      int copied = 0;

      for (int i = 0; i < data.length; i++) {
         switch (data[i]) {
            default:
               groomed[copied++] = data[i];
               break;
            case 10:
            case 32:
         }
      }

      return groomed;
   }
}
