// Golden vectors for rt::jchar, from the JDK 21 java.lang.Character.
//
//   java GenJcharGolden.java        > jchar_golden.txt
//   java GenJcharGolden.java table  (the TYPE_RUNS table embedded in src/rt/jchar.rs)
//
// Golden lines are "first last type flags digit10 digit36" for each maximal run
// of chars 0..0xFFFF sharing all values (hex char codes, decimal fields).
// Flag bits: 0 isJavaIdentifierStart, 1 isJavaIdentifierPart, 2 isIdentifierIgnorable,
// 3 isDigit, 4 isLetter, 5 isLetterOrDigit, 6 isWhitespace, 7 isISOControl,
// 8 isSurrogate, 9 isHighSurrogate, 10 isLowSurrogate.
public class GenJcharGolden {
  public static void main(String[] args) {
    if (args.length > 0 && args[0].equals("table")) {
      StringBuilder row = new StringBuilder();
      int prev = -1, n = 0;
      for (int c = 0; c <= 0xFFFF; c++) {
        int t = Character.getType((char) c);
        if (t != prev) {
          row.append(String.format("(0x%04X, %d), ", c, t));
          if (++n % 7 == 0) {
            System.out.println("    " + row.toString().trim());
            row.setLength(0);
          }
          prev = t;
        }
      }
      if (row.length() > 0) System.out.println("    " + row.toString().trim());
      System.err.println(n + " runs");
      return;
    }
    String prev = null;
    int first = 0;
    for (int c = 0; c <= 0x10000; c++) {
      String cur = c <= 0xFFFF ? describe((char) c) : null;
      if (prev != null && !prev.equals(cur)) {
        System.out.println(Integer.toHexString(first) + " " + Integer.toHexString(c - 1) + " " + prev);
        first = c;
      } else if (prev == null) {
        first = c;
      }
      prev = cur;
    }
  }

  static String describe(char ch) {
    int flags = 0;
    if (Character.isJavaIdentifierStart(ch)) flags |= 1;
    if (Character.isJavaIdentifierPart(ch)) flags |= 1 << 1;
    if (Character.isIdentifierIgnorable(ch)) flags |= 1 << 2;
    if (Character.isDigit(ch)) flags |= 1 << 3;
    if (Character.isLetter(ch)) flags |= 1 << 4;
    if (Character.isLetterOrDigit(ch)) flags |= 1 << 5;
    if (Character.isWhitespace(ch)) flags |= 1 << 6;
    if (Character.isISOControl(ch)) flags |= 1 << 7;
    if (Character.isSurrogate(ch)) flags |= 1 << 8;
    if (Character.isHighSurrogate(ch)) flags |= 1 << 9;
    if (Character.isLowSurrogate(ch)) flags |= 1 << 10;
    return Character.getType(ch) + " " + flags + " " + Character.digit(ch, 10) + " " + Character.digit(ch, 36);
  }
}
