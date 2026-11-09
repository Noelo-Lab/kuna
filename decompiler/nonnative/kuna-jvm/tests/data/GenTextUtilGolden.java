// Golden vectors for the context-free TextUtil methods, from Vineflower 1.12.0's own TextUtil:
//
//   javac -cp vineflower-1.12.0.jar -d /tmp/gen GenTextUtilGolden.java
//   java -cp vineflower-1.12.0.jar:/tmp/gen GenTextUtilGolden > text_util_golden.txt
//
// "P first last 0|1": isPrintableUnicode over maximal runs of 0..0xFFFF (hex);
// "U value literal": charToUnicodeLiteral; "I opcode name": getInstructionName.
import org.jetbrains.java.decompiler.util.TextUtil;

public class GenTextUtilGolden {
  public static void main(String[] args) {
    StringBuilder out = new StringBuilder();
    int first = 0;
    boolean prev = TextUtil.isPrintableUnicode((char) 0);
    for (int c = 1; c <= 0x10000; c++) {
      boolean cur = c <= 0xFFFF && TextUtil.isPrintableUnicode((char) c);
      if (c > 0xFFFF || cur != prev) {
        out.append("P ").append(Integer.toHexString(first)).append(' ').append(Integer.toHexString(c - 1)).append(' ').append(prev ? 1 : 0).append('\n');
        first = c;
        prev = cur;
      }
    }
    int[] values = {0, 1, 0x1f, 0x7f, 0xa0, 0xff, 0x100, 0x7ff, 0xfff, 0x1000, 0xd800, 0xffff, 0x10000, 0x12345, -1, Integer.MIN_VALUE};
    for (int v : values) out.append("U ").append(v).append(' ').append(TextUtil.charToUnicodeLiteral(v)).append('\n');
    for (int op = 0; op <= 201; op++) out.append("I ").append(op).append(' ').append(TextUtil.getInstructionName(op)).append('\n');
    System.out.print(out);
  }
}
