// Differential traces for the TextBuffer port, recorded on Vineflower 1.12.0's own TextBuffer:
//
//   javac -cp vineflower-1.12.0.jar -d GEN_DIR GenTextBufferGolden.java
//   java -cp vineflower-1.12.0.jar:GEN_DIR GenTextBufferGolden > text_buffer_golden.txt
//
// A scenario starts with "TB <unix 0|1> <indent> <preferred line length>"; every line is one
// operation on one of five buffers (the fifth gathers the others at the end), "op args = result" where it has a result. String arguments
// are "x" + hex UTF-16 units; results escape \ \n \r \t and every other unit outside 0x20..0x7e.
import org.jetbrains.java.decompiler.main.ClassesProcessor;
import org.jetbrains.java.decompiler.main.DecompilerContext;
import org.jetbrains.java.decompiler.main.extern.IFernflowerLogger;
import org.jetbrains.java.decompiler.main.extern.IFernflowerPreferences;
import org.jetbrains.java.decompiler.struct.StructContext;
import org.jetbrains.java.decompiler.util.TextBuffer;
import org.jetbrains.java.decompiler.util.token.ClassTextToken;
import org.jetbrains.java.decompiler.util.token.TextToken;

import java.lang.reflect.Field;
import java.util.*;

public class GenTextBufferGolden {
  static final Random R = new Random(0x7E47B0FL);
  static final StringBuilder OUT = new StringBuilder();
  static final TextBuffer[] BUFS = new TextBuffer[5];
  static final int[] DEPTH = new int[5];
  static final String[] WORDS = {"alpha", "betaGamma", "x", "veryLongIdentifierName", "café", "日本語", "😀",
    "\"s p\"", "a.b", "1234567890", "  ", "{", "}", "return", "new Object()", "\t"};
  static final String[][] CLASSES = {{"String", "java/lang/String"}, {"Map.Entry", "java/util/Map$Entry"}, {"Outer.Inner.Deep", "a/b/Outer$Inner$Deep"},
    {"Map.Entry<K, V>", "java/util/Map$Entry"}, {"List<String>", "java/util/List"}, {"Café.Été", "p/Café$Été"}, {"  ", "q/Blank"}};

  static void line(String s) { OUT.append(s).append('\n'); }

  static String enc(String s) {
    StringBuilder b = new StringBuilder("x");
    for (int i = 0; i < s.length(); i++) b.append(String.format("%04x", (int) s.charAt(i)));
    return b.toString();
  }

  static String esc(String s) {
    StringBuilder b = new StringBuilder();
    for (int i = 0; i < s.length(); i++) {
      char c = s.charAt(i);
      switch (c) {
        case '\\': b.append("\\\\"); break;
        case '\n': b.append("\\n"); break;
        case '\r': b.append("\\r"); break;
        case '\t': b.append("\\t"); break;
        default:
          if (c < 0x20 || c > 0x7e) b.append(String.format("\\u%04x", (int) c)); else b.append(c);
      }
    }
    return b.toString();
  }

  static String tokens(TextBuffer b) {
    StringBuilder s = new StringBuilder();
    for (TextToken t : b.getTokens()) {
      if (s.length() > 0) s.append(';');
      s.append(t.getStart()).append(':').append(t.getLength()).append(':').append(t.isDeclaration() ? 1 : 0).append(':').append(esc(((ClassTextToken) t).qualifiedName));
    }
    return s.length() == 0 ? "-" : s.toString();
  }

  static String word() { return WORDS[R.nextInt(WORDS.length)]; }

  public static void main(String[] args) throws Exception {
    Field f = sun.misc.Unsafe.class.getDeclaredField("theUnsafe");
    f.setAccessible(true);
    sun.misc.Unsafe unsafe = (sun.misc.Unsafe) f.get(null);
    StructContext sc = (StructContext) unsafe.allocateInstance(StructContext.class);
    ClassesProcessor cp = (ClassesProcessor) unsafe.allocateInstance(ClassesProcessor.class);
    IFernflowerLogger logger = new IFernflowerLogger() {
      public void writeMessage(String message, Severity severity) { }
      public void writeMessage(String message, Severity severity, Throwable t) { }
    };
    String[] indents = {"   ", "\t", "  ", "    "};
    int[] lengths = {20, 30, 40, 60, 160, 8};
    for (int scenario = 0; scenario < 60; scenario++) {
      boolean unix = scenario % 4 != 3;
      String indent = indents[scenario % indents.length];
      int pll = lengths[scenario % lengths.length];
      Map<String, Object> props = new HashMap<>();
      props.put(IFernflowerPreferences.NEW_LINE_SEPARATOR, unix ? "1" : "0");
      props.put(IFernflowerPreferences.INDENT_STRING, indent);
      props.put(IFernflowerPreferences.PREFERRED_LINE_LENGTH, Integer.toString(pll));
      DecompilerContext.setCurrentContext(new DecompilerContext(props, logger, sc, cp, null));
      line("TB " + (unix ? 1 : 0) + " " + enc(indent) + " " + pll);
      run(scenario);
    }
    System.out.print(OUT);
  }

  static void ap(int b, String s) { BUFS[b].append(s); line("ap " + b + " " + enc(s)); }

  static void expr(int b, int level) {
    int base = R.nextInt(3), extra = 1 + R.nextInt(2);
    BUFS[b].pushNewlineGroup(base, extra); DEPTH[b]++;
    line("push " + b + " " + base + " " + extra);
    ap(b, word() + "(");
    int n = R.nextInt(5);
    for (int i = 0; i < n; i++) {
      if (i > 0) { ap(b, ","); String alt = R.nextInt(4) == 0 ? "" : " "; BUFS[b].appendPossibleNewline(alt); line("pna " + b + " " + enc(alt)); }
      else { BUFS[b].appendPossibleNewline(); line("pn " + b); }
      if (level < 3 && R.nextInt(3) == 0) expr(b, level + 1);
      else if (R.nextInt(5) == 0) cls(b);
      else ap(b, word());
      if (R.nextInt(12) == 0) { BUFS[b].appendLineSeparator(); line("ls " + b); }
    }
    if (R.nextInt(3) == 0) { BUFS[b].appendPossibleNewline("", true); line("pnd " + b + " " + enc("")); }
    ap(b, ")");
    BUFS[b].popNewlineGroup(); DEPTH[b]--;
    line("pop " + b);
  }

  static void cls(int b) {
    String[] c = CLASSES[R.nextInt(CLASSES.length)];
    switch (R.nextInt(3)) {
      case 0: { boolean decl = R.nextBoolean(); BUFS[b].appendClass(c[0], decl, c[1]); line("cls " + b + " " + enc(c[0]) + " " + (decl ? 1 : 0) + " " + enc(c[1])); break; }
      case 1: BUFS[b].appendAllClasses(c[0], c[1]); line("aac " + b + " " + enc(c[0]) + " " + enc(c[1])); break;
      default: {
        int idx = R.nextInt(BUFS[b].length() + 1), len = R.nextInt(6);
        BUFS[b].addClassToken(idx, len, c[1]); line("act " + b + " " + idx + " " + len + " " + enc(c[1]));
        BUFS[b].addAllClassTokens(idx, c[0], c[1]); line("aat " + b + " " + idx + " " + enc(c[0]) + " " + enc(c[1]));
      }
    }
  }

  static void run(int scenario) {
    for (int i = 0; i < BUFS.length; i++) { BUFS[i] = new TextBuffer(); DEPTH[i] = 0; line("new " + i); }
    int steps = 60 + R.nextInt(80);
    for (int step = 0; step < steps; step++) {
      int b = R.nextInt(4);
      int len = BUFS[b].length();
      switch (R.nextInt(24)) {
        case 0: case 1: case 2: ap(b, word()); break;
        case 3: { char c = "a;( é".charAt(R.nextInt(5)); BUFS[b].append(c); line("ac " + b + " " + enc(String.valueOf(c))); break; }
        case 4: { int n = R.nextInt(2001) - 1000; BUFS[b].append(n); line("ai " + b + " " + n); break; }
        case 5: BUFS[b].appendLineSeparator(); line("ls " + b); break;
        case 6: { int n = R.nextInt(4); BUFS[b].appendIndent(n); line("in " + b + " " + n); break; }
        case 7: case 8: case 9: expr(b, 0); break;
        case 10: { int base = R.nextInt(3), extra = R.nextInt(3); BUFS[b].pushNewlineGroup(base, extra); DEPTH[b]++; line("push " + b + " " + base + " " + extra); break; }
        case 11: if (DEPTH[b] > 0) { BUFS[b].popNewlineGroup(); DEPTH[b]--; line("pop " + b); } else { BUFS[b].appendPossibleNewline(" "); line("pna " + b + " " + enc(" ")); } break;
        case 12: { String s = word(); BUFS[b].prepend(s); line("pre " + b + " " + enc(s)); break; }
        case 13: BUFS[b].enclose("(", ")"); line("enc " + b + " " + enc("(") + " " + enc(")")); break;
        case 14: if (R.nextInt(3) == 0) { int n = R.nextInt(Math.min(len, 6) + 1); BUFS[b].setStart(n); line("ss " + b + " " + n); } break;
        case 15: if (R.nextInt(3) == 0) { int n = Math.max(0, len - R.nextInt(8)); BUFS[b].setLength(n); line("sl " + b + " " + n); } break;
        case 16: case 17: cls(b); break;
        case 18: { int off = R.nextInt(50); if (R.nextInt(4) == 0) { BUFS[b].addStartBytecodeMapping(off); line("bsm " + b + " " + off); } else { BUFS[b].addBytecodeMapping(off); line("bm " + b + " " + off); } break; }
        case 19: case 20: {
          int c = (b + 1 + R.nextInt(3)) % 4;
          if (DEPTH[c] == 0) {
            if (R.nextBoolean()) { BUFS[b].append(BUFS[c]); line("apb " + b + " " + c); }
            else { String k = "m" + R.nextInt(3); BUFS[b].append(BUFS[c], "pkg/C", k); line("apbm " + b + " " + c + " " + enc("pkg/C") + " " + enc(k)); }
            if (R.nextInt(3) == 0) { BUFS[c] = new TextBuffer(); line("new " + c); }
          }
          break;
        }
        case 21: line("q " + b + " = " + BUFS[b].length() + " " + BUFS[b].countLines() + " " + BUFS[b].count(" ", 1) + " " + (BUFS[b].containsOnlyWhitespaces() ? 1 : 0) + " " + BUFS[b].countChars('a')); break;
        case 22: { int i = R.nextInt(len + 1); line("pos " + b + " " + i + " = " + BUFS[b].getPos(i)); break; }
        case 23: line("str " + b + " = " + esc(BUFS[b].convertToStringAndAllowDataDiscard()) + " " + tokens(BUFS[b])); break;
      }
    }
    for (int i = 0; i < BUFS.length; i++) {
      while (DEPTH[i] > 0) { BUFS[i].popNewlineGroup(); DEPTH[i]--; line("pop " + i); }
      line("str " + i + " = " + esc(BUFS[i].convertToStringAndAllowDataDiscard()) + " " + tokens(BUFS[i]));
    }
    for (int i = 0; i < 4; i++) {
      BUFS[4].append(BUFS[i], "pkg/C", "m" + i);
      line("apbm 4 " + i + " " + enc("pkg/C") + " " + enc("m" + i));
      BUFS[4].appendLineSeparator();
      line("ls 4");
    }
    for (int i = 4; i >= 0; i--) {
      String r;
      try { BUFS[i].reformat(); r = "OK"; } catch (RuntimeException e) { r = "EXC"; }
      line("ref " + i + " = " + r);
      line("str " + i + " = " + esc(BUFS[i].convertToStringAndAllowDataDiscard()) + " " + tokens(BUFS[i]));
    }
    if (scenario % 5 == 0) {
      int[] mapping = {3, 1, 7, 1, 2, 2, 9, 3, 4, 5};
      BUFS[0].dumpOriginalLineNumbers(mapping);
      line("dol 0 3,1,7,1,2,2,9,3,4,5");
      line("str 0 = " + esc(BUFS[0].convertToStringAndAllowDataDiscard()) + " " + tokens(BUFS[0]));
    }
  }
}
