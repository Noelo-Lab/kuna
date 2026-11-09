// Golden vectors for tests/class_model.rs: the class nodes ClassesProcessor.loadClasses
// builds for classes/ (own) + lib/ (library), lambda nodes, ImportCollector short names
// and imports, BytecodeSourceMapper.dumpMapping and ClassWriter.writeException.
// Run through gen.sh.
import java.io.File;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

import org.jetbrains.java.decompiler.main.ClassWriter;
import org.jetbrains.java.decompiler.main.ClassesProcessor.ClassNode;
import org.jetbrains.java.decompiler.main.DecompilerContext;
import org.jetbrains.java.decompiler.main.Fernflower;
import org.jetbrains.java.decompiler.main.collectors.BytecodeMappingTracer;
import org.jetbrains.java.decompiler.main.collectors.BytecodeSourceMapper;
import org.jetbrains.java.decompiler.main.collectors.ImportCollector;
import org.jetbrains.java.decompiler.main.extern.IFernflowerLogger;
import org.jetbrains.java.decompiler.main.extern.IFernflowerPreferences;
import org.jetbrains.java.decompiler.struct.StructClass;
import org.jetbrains.java.decompiler.struct.StructMethod;
import org.jetbrains.java.decompiler.struct.attr.StructGeneralAttribute;
import org.jetbrains.java.decompiler.util.TextBuffer;

public class GenClassModelOracle {
  static String esc(String s) {
    if (s == null) return "\\N";
    return s.replace("\\", "\\\\").replace("\t", "\\t").replace("\n", "\\n");
  }

  static String node(ClassNode n) {
    List<String> nested = new ArrayList<>();
    for (ClassNode c : n.nested) nested.add(c.classStruct.qualifiedName);
    List<String> enclosing = new ArrayList<>(n.enclosingClasses);
    enclosing.sort(null);
    return String.join("\t", n.classStruct.qualifiedName, n.type.name(), esc(n.simpleName),
      Integer.toString(n.access), n.parent == null ? "\\N" : n.parent.classStruct.qualifiedName,
      String.join(",", nested), esc(n.enclosingMethod), String.join(",", enclosing),
      n.anonymousClassType == null ? "\\N" : n.anonymousClassType.value, n.toString());
  }

  static String lambda(ClassNode n) {
    ClassNode.LambdaInformation l = n.lambdaInformation;
    return String.join("\t", n.type.name(), esc(n.simpleName), l.method_name, l.method_descriptor,
      l.content_class_name, l.content_method_name, l.content_method_descriptor,
      Integer.toString(l.content_method_invocation_type), l.content_method_key,
      Boolean.toString(l.is_method_reference), Boolean.toString(l.is_content_method_static),
      n.anonymousClassType.value, n.toString());
  }

  public static void main(String[] args) throws Exception {
    File dir = new File(args[0]);
    Map<String, Object> options = new HashMap<>();
    options.put(IFernflowerPreferences.INCLUDE_JAVA_RUNTIME, "0");
    options.put(IFernflowerPreferences.DUMP_EXCEPTION_ON_ERROR, "0");
    IFernflowerLogger logger = new IFernflowerLogger() {
      @Override public void writeMessage(String message, Severity severity) { }
      @Override public void writeMessage(String message, Severity severity, Throwable t) { }
    };
    Fernflower fernflower = new Fernflower(null, options, logger);
    fernflower.addSource(new File(dir, "classes"));
    fernflower.addLibrary(new File(dir, "lib"));
    DecompilerContext.getClassProcessor().loadClasses(null);

    Map<String, ClassNode> roots = DecompilerContext.getClassProcessor().getMapRootClasses();
    List<String> names = new ArrayList<>(roots.keySet());
    names.sort(null);
    System.out.println("== nodes");
    for (String name : names) System.out.println(node(roots.get(name)));
    ClassNode outer = roots.get("p/Outer");
    List<String> all = new ArrayList<>();
    for (ClassNode n : outer.getAllNested()) all.add(n.classStruct.qualifiedName);
    System.out.println("all\t" + String.join(",", all));

    System.out.println("== lambdas");
    StructClass cl = outer.classStruct;
    System.out.println(lambda(new ClassNode("p/Outer", "lambda$new$0", "()V", 6, "java/lang/Runnable", "run", "()V", cl)));
    System.out.println(lambda(new ClassNode("java/lang/Object", "toString", "()Ljava/lang/String;", 5,
      "java/util/function/Supplier", "get", "()Ljava/lang/Object;", cl)));
    System.out.println(lambda(new ClassNode("p/Outer", "run", "()V", 5, "java/lang/Runnable", "run", "()V", cl)));

    System.out.println("== short names");
    ImportCollector imports = new ImportCollector(outer);
    DecompilerContext.setProperty(DecompilerContext.CURRENT_CLASS_NODE, outer);
    String[] queries = {
      "java.util.List", "java.awt.List", "java.lang.String", "p.Outer", "p.Outer$A", "p.Outer$A$1",
      "p.Outer$1Local", "q.Outer", "x.y.A", "p.Outer.B", "q.Other$A", "java.util.concurrent.Future",
      "a.b.Zed", "Default", "java.util.List", "x.field"
    };
    for (String q : queries) System.out.println("short\t" + q + "\t" + esc(imports.getShortName(q)));
    System.out.println("short\tjava.util.Map\tnot-imported\t" + esc(imports.getShortName("java.util.Map", false)));
    try (ImportCollector.Lock lock = imports.lock()) {
      System.out.println("short\tjava.util.Set\tlocked\t" + esc(imports.getShortName("java.util.Set")));
    }
    System.out.println("short\tjava.awt.Set\t" + esc(imports.getShortName("java.awt.Set")));
    for (String q : new String[] {"x.field", "java.util.List", "p.Outer$A$1"}) {
      System.out.println("context\t" + q + "\t" + esc(imports.getShortNameInClassContext(q)));
    }
    TextBuffer buffer = new TextBuffer();
    imports.writeImports(buffer, true);
    System.out.println("imports\t" + esc(buffer.convertToStringAndAllowDataDiscard()));
    TextBuffer header = new TextBuffer();
    new ClassWriter().writeClassHeader(cl, header, imports);
    System.out.println("header\t" + esc(header.convertToStringAndAllowDataDiscard()));
    TextBuffer packageInfo = new TextBuffer();
    ImportCollector empty = new ImportCollector(roots.get("p/Outer$B"));
    new ClassWriter().writeClassHeader(roots.get("p/Outer$B").classStruct, packageInfo, empty);
    System.out.println("header-empty\t" + esc(packageInfo.convertToStringAndAllowDataDiscard()));

    System.out.println("== mapper");
    StructMethod run = cl.getMethod("run", "()V");
    BytecodeMappingTracer tracer = new BytecodeMappingTracer(3);
    tracer.setLineNumberTable(run.getAttribute(StructGeneralAttribute.ATTRIBUTE_LINE_NUMBER_TABLE));
    tracer.addMapping(0);
    tracer.incrementCurrentSourceLine();
    tracer.addMapping(8);
    BytecodeSourceMapper mapper = new BytecodeSourceMapper();
    TextBuffer none = new TextBuffer();
    mapper.dumpMapping(none, false);
    System.out.println("empty\t" + esc(none.convertToStringAndAllowDataDiscard()));
    mapper.addTracer("p/Outer", "run ()V", tracer);
    mapper.addMapping("p/Outer", "<init> ()V", 4, 7);
    mapper.addMapping("p/Outer", "run ()V", 8, 99);
    mapper.addMapping("q/Other", "x ()V", 255, 5);
    mapper.addTotalOffset(2);
    for (boolean hex : new boolean[] {false, true}) {
      TextBuffer out = new TextBuffer();
      mapper.dumpMapping(out, hex);
      System.out.println("dump\t" + hex + "\t" + esc(out.convertToStringAndAllowDataDiscard()));
    }
    int[] lines = mapper.getOriginalLinesMapping();
    StringBuilder sb = new StringBuilder();
    for (int line : lines) sb.append(sb.length() == 0 ? "" : ",").append(line);
    System.out.println("lines\t" + sb);
    TextBuffer after = new TextBuffer();
    mapper.dumpMapping(after, false);
    System.out.println("dump\tafter\t" + esc(after.convertToStringAndAllowDataDiscard()));

    System.out.println("== writer");
    TextBuffer exception = new TextBuffer();
    ClassWriter.writeException(exception, new RuntimeException("boom"));
    System.out.println("exception\t" + esc(exception.convertToStringAndAllowDataDiscard()));
    System.out.println("error-comment\t" + esc(String.join("\n", ClassWriter.getErrorComment())));
  }
}
