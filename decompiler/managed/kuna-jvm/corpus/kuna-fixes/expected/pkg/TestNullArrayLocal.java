package pkg;

import java.io.UnsupportedEncodingException;

public class TestNullArrayLocal {
   String charset = "UTF-8";

   byte[] encode(String var1) {
      byte[] var2 = null;

      try {
         var2 = var1.getBytes(this.charset);
      } catch (UnsupportedEncodingException var4) {
         throw new IllegalStateException(var4);
      }

      return var2;
   }
}
