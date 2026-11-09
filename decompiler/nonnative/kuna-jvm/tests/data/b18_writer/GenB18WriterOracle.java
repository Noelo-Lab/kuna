// Golden vectors for tests/b18_class_writer.rs: ClassWriter.toValidJavaIdentifier, and
// ClassWriter.writeMethod on methods whose wrapper carries a decompile error (no root),
// with dump-exception-on-error off and dump-bytecode-on-error on, so the output is the
// method header plus dumpError's bytecode listing. Only <clinit> and the no-argument
// constructor are written: their headers need no type names. Run through gen.sh.
import java.io.File;
import java.lang.reflect.Field;
import java.util.HashMap;
import java.util.Map;

import org.jetbrains.java.decompiler.main.ClassWriter;
import org.jetbrains.java.decompiler.main.ClassesProcessor.ClassNode;
import org.jetbrains.java.decompiler.main.DecompilerContext;
import org.jetbrains.java.decompiler.main.Fernflower;
import org.jetbrains.java.decompiler.main.collectors.CounterContainer;
import org.jetbrains.java.decompiler.main.extern.IFernflowerLogger;
import org.jetbrains.java.decompiler.main.extern.IFernflowerPreferences;
import org.jetbrains.java.decompiler.main.rels.ClassWrapper;
import org.jetbrains.java.decompiler.main.rels.MethodWrapper;
import org.jetbrains.java.decompiler.modules.decompiler.vars.VarProcessor;
import org.jetbrains.java.decompiler.struct.StructClass;
import org.jetbrains.java.decompiler.struct.StructMethod;
import org.jetbrains.java.decompiler.struct.gen.MethodDescriptor;
import org.jetbrains.java.decompiler.util.InterpreterUtil;
import org.jetbrains.java.decompiler.util.TextBuffer;
import org.jetbrains.java.decompiler.util.collections.VBStyleCollection;

public class GenB18WriterOracle {
  static String esc(String s) {
    if (s == null) return "\\N";
    StringBuilder sb = new StringBuilder();
    for (int i = 0; i < s.length(); i++) {
      char c = s.charAt(i);
      if (c == '\\') sb.append("\\\\");
      else if (c == '\t') sb.append("\\t");
      else if (c == '\n') sb.append("\\n");
      else if (c < 0x20 || c > 0x7e) sb.append(String.format("\\u%04x", (int) c));
      else sb.append(c);
    }
    return sb.toString();
  }

  static final String[] IDENTIFIERS = {
    "abc", "1abc", "a-b", "ünï", "a\u0000b", "$x", "_", "", "a b", "😀x",
    "x😀", "​x", "x​", "<init>", "a.b/c", "٠x", "x٠",
  };

  public static void main(String[] args) throws Exception {
    File dir = new File(args[0]);

    for (String id : IDENTIFIERS) {
      System.out.println("ident\t" + esc(id) + "\t" + esc(ClassWriter.toValidJavaIdentifier(id)));
    }
    System.out.println("ident\t\\N\t" + esc(ClassWriter.toValidJavaIdentifier(null)));

    Map<String, Object> options = new HashMap<>();
    options.put(IFernflowerPreferences.INCLUDE_JAVA_RUNTIME, "0");
    options.put(IFernflowerPreferences.DUMP_EXCEPTION_ON_ERROR, "0");
    options.put(IFernflowerPreferences.DUMP_BYTECODE_ON_ERROR, "1");
    IFernflowerLogger logger = new IFernflowerLogger() {
      @Override public void writeMessage(String message, Severity severity) { }
      @Override public void writeMessage(String message, Severity severity, Throwable t) { }
    };
    Fernflower fernflower = new Fernflower(null, options, logger);
    fernflower.addSource(new File(dir, "classes"));

    StructClass cl = DecompilerContext.getStructContext().getClass("w/Dump");
    ClassNode node = new ClassNode(ClassNode.Type.ROOT, cl);
    node.simpleName = "Dump";
    node.access = cl.getAccessFlags();
    ClassWrapper wrapper = new ClassWrapper(node);
    Field field = ClassNode.class.getDeclaredField("wrapper");
    field.setAccessible(true);
    field.set(node, wrapper);
    DecompilerContext.setProperty(DecompilerContext.CURRENT_CLASS_NODE, node);

    VBStyleCollection<StructMethod, String> methods = cl.getMethods();
    for (int i = 0; i < methods.size(); i++) {
      StructMethod mt = methods.get(i);
      MethodDescriptor md = MethodDescriptor.parseDescriptor(mt.getDescriptor());
      MethodWrapper mw = new MethodWrapper(null, new VarProcessor(mt, md), mt, cl, new CounterContainer());
      mw.decompileError = new RuntimeException("boom");
      wrapper.getMethods().addWithKey(mw, InterpreterUtil.makeUniqueKey(mt.getName(), mt.getDescriptor()));
    }

    ClassWriter writer = new ClassWriter();
    for (int i = 0; i < methods.size(); i++) {
      StructMethod mt = methods.get(i);
      String key = mt.getName() + mt.getDescriptor();
      if (!key.equals("<clinit>()V") && !key.equals("<init>()V")) continue;
      TextBuffer buffer = new TextBuffer();
      boolean shown = writer.writeMethod(node, mt, i, buffer, 1);
      System.out.println("method\t" + key + "\t" + shown + "\t" + esc(buffer.convertToStringAndAllowDataDiscard()));
    }

    // Typed headers (primitive types only), with and without MethodParameters names.
    for (String useNames : new String[] {"1", "0"}) {
      DecompilerContext.setProperty(IFernflowerPreferences.USE_METHOD_PARAMETERS, useNames);
      DecompilerContext.setProperty(IFernflowerPreferences.DUMP_BYTECODE_ON_ERROR, "0");
      for (int i = 0; i < methods.size(); i++) {
        StructMethod mt = methods.get(i);
        String key = mt.getName() + mt.getDescriptor();
        if (mt.getName().startsWith("<") || mt.getName().startsWith("lambda$")) continue;
        TextBuffer buffer = new TextBuffer();
        boolean shown = writer.writeMethod(node, mt, i, buffer, 1);
        System.out.println("typed\t" + useNames + "\t" + key + "\t" + shown + "\t" + esc(buffer.convertToStringAndAllowDataDiscard()));
      }
    }
  }
}
