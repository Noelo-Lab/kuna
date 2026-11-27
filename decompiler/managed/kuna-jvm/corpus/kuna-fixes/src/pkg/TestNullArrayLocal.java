package pkg;

import java.io.UnsupportedEncodingException;

public class TestNullArrayLocal {
    String charset = "UTF-8";

    byte[] encode(String text) {
        byte[] bytes = null;
        try {
            bytes = text.getBytes(charset);
        } catch (UnsupportedEncodingException e) {
            throw new IllegalStateException(e);
        }
        return bytes;
    }
}
