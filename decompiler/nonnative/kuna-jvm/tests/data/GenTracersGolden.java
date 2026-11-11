// Golden vector for TextBuffer's bytecode-mapping tracers after reformat() and for the token dump
// visitor, from Vineflower 1.12.0's own TextBuffer:
//
//   javac -cp vineflower-1.12.0.jar -d GEN_DIR GenTracersGolden.java
//   java -cp vineflower-1.12.0.jar:GEN_DIR GenTracersGolden > tracers_golden.txt
import org.jetbrains.java.decompiler.main.*;
import org.jetbrains.java.decompiler.main.collectors.BytecodeMappingTracer;
import org.jetbrains.java.decompiler.main.extern.*;
import org.jetbrains.java.decompiler.struct.StructContext;
import org.jetbrains.java.decompiler.util.*;
import org.jetbrains.java.decompiler.util.token.TextTokenDumpVisitor;
import java.util.*;
public class GenTracersGolden {
  public static void main(String[] a) throws Exception {
    java.lang.reflect.Field f = sun.misc.Unsafe.class.getDeclaredField("theUnsafe"); f.setAccessible(true);
    sun.misc.Unsafe u = (sun.misc.Unsafe) f.get(null);
    Map<String, Object> p = new HashMap<>();
    p.put(IFernflowerPreferences.NEW_LINE_SEPARATOR, "1"); p.put(IFernflowerPreferences.INDENT_STRING, "   "); p.put(IFernflowerPreferences.PREFERRED_LINE_LENGTH, "160");
    IFernflowerLogger l = new IFernflowerLogger() { public void writeMessage(String m, Severity s) {} public void writeMessage(String m, Severity s, Throwable t) {} };
    DecompilerContext.setCurrentContext(new DecompilerContext(p, l, (StructContext) u.allocateInstance(StructContext.class), (ClassesProcessor) u.allocateInstance(ClassesProcessor.class), null));
    TextBuffer cls = new TextBuffer();
    for (int m = 0; m < 3; m++) {
      TextBuffer code = new TextBuffer();
      for (int i = 0; i < 4; i++) {
        code.appendIndent(1);
        code.addBytecodeMapping(i * 3 + m);
        code.pushNewlineGroup(1, 1);
        code.append("call" + i + "(");
        for (int k = 0; k < 2 + m * 6; k++) { if (k > 0) { code.append(","); code.appendPossibleNewline(" "); } else code.appendPossibleNewline(); code.append("argumentNumber" + k); code.addBytecodeMapping(100 + i * 10 + k); }
        code.append(");");
        code.popNewlineGroup();
        code.appendLineSeparator();
        if (i == 1) code.addStartBytecodeMapping(77);
      }
      cls.appendClass("Foo", true, "pkg/Foo").append(" m" + m + "() {").appendLineSeparator();
      cls.append(code, "pkg/Foo", "m" + m + " ()V");
      cls.append("}").appendLineSeparator();
    }
    cls.reformat();
    StringBuilder out = new StringBuilder();
    cls.getTracers().forEach((k, t) -> out.append(k.a).append('|').append(k.b).append('|').append(new TreeMap<>(t.getMapping())).append('\n'));
    TextBuffer[] holder = {cls};
    cls.visitTokens(TextTokenVisitor.createVisitor(next -> new TextTokenDumpVisitor(next, holder[0])));
    out.append(cls.convertToStringAndAllowDataDiscard());
    System.out.print(out);
  }
}
