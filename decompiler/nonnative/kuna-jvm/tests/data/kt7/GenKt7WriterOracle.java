// Golden vectors for tests/kt7_kotlin_writer.rs: the Kotlin plugin's own KotlinWriter
// (org.vineflower.kotlin, bundled in the Vineflower 1.12.0 jar), on the pieces that need
// no other Kotlin-plugin body: toValidKotlinIdentifier, appendModifiers (private, by
// reflection), appendJvmAnnotations, processParameterAnnotations, isNullable,
// searchForMethod, hideConstructor, appendTypeParameters, appendAnnotations on members
// whose only declaration annotations are the ones it filters out, writeField on fields
// without other declaration annotations, and writeMethod on methods whose wrapper
// carries a decompile error (header + dumpError's bytecode listing). Run through gen.sh.
import java.io.File;
import java.lang.reflect.Field;
import java.lang.reflect.Method;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

import org.jetbrains.java.decompiler.code.CodeConstants;
import org.jetbrains.java.decompiler.main.ClassesProcessor.ClassNode;
import org.jetbrains.java.decompiler.main.DecompilerContext;
import org.jetbrains.java.decompiler.main.Fernflower;
import org.jetbrains.java.decompiler.main.collectors.CounterContainer;
import org.jetbrains.java.decompiler.main.collectors.ImportCollector;
import org.jetbrains.java.decompiler.main.extern.IFernflowerLogger;
import org.jetbrains.java.decompiler.main.extern.IFernflowerPreferences;
import org.jetbrains.java.decompiler.main.plugins.PluginSources;
import org.jetbrains.java.decompiler.main.rels.ClassWrapper;
import org.jetbrains.java.decompiler.main.rels.MethodWrapper;
import org.jetbrains.java.decompiler.modules.decompiler.exps.TypeAnnotation;
import org.jetbrains.java.decompiler.modules.decompiler.vars.VarProcessor;
import org.jetbrains.java.decompiler.struct.StructClass;
import org.jetbrains.java.decompiler.struct.StructField;
import org.jetbrains.java.decompiler.struct.StructMember;
import org.jetbrains.java.decompiler.struct.StructMethod;
import org.jetbrains.java.decompiler.struct.gen.MethodDescriptor;
import org.jetbrains.java.decompiler.struct.gen.generics.GenericClassDescriptor;
import org.jetbrains.java.decompiler.struct.gen.generics.GenericMethodDescriptor;
import org.jetbrains.java.decompiler.util.InterpreterUtil;
import org.jetbrains.java.decompiler.util.TextBuffer;
import org.vineflower.kotlin.KotlinWriter;

public class GenKt7WriterOracle {
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

  // Keep in sync with IDENTIFIERS in tests/kt7_kotlin_writer.rs.
  static final String[] IDENTIFIERS = {
    "as", "break", "class", "continue", "do", "else", "false", "for", "fun", "if", "in",
    "interface", "is", "null", "object", "package", "return", "super", "this", "throw",
    "true", "try", "typealias", "typeof", "val", "var", "when", "while",
    "by", "catch", "constructor", "delegate", "dynamic", "field", "file", "finally", "get",
    "import", "init", "param", "property", "receiver", "set", "setparam", "value", "where",
    "abstract", "actual", "annotation", "companion", "const", "crossinline", "data", "enum",
    "expect", "external", "final", "infix", "inline", "inner", "internal", "lateinit",
    "noinline", "open", "operator", "out", "override", "private", "protected", "public",
    "reified", "sealed", "suspend", "tailrec", "vararg", "it",
    "int", "static", "new", "void", "boolean", "default", "case", "switch", "goto",
    "abc", "Abc", "_", "__", "_x", "x1", "1abc", "", " ", "a b", "a-b", "a.b", "a/b",
    "<init>", "<clinit>", "$", "$x", "x$", "a$b", "$this$foo", "`", "a`b", "`a`", "``",
    "a`$", "$`", "1`", "ünï", "café", "π", "ß", "日本語",
    "a\u0000b", "\u0000", "😀x", "x😀", "​x", "x​",
    "٠x", "x٠", "a\tb", "a\nb", "a b", "ª", "x·", "ʰx",
    "Ab$Companion", "access$getX$p", "get-impl", "component1", "x;y", "a:b", "A[]",
  };

  static final int[] FLAG_SETS = {
    0, CodeConstants.ACC_PUBLIC, CodeConstants.ACC_PRIVATE, CodeConstants.ACC_PROTECTED,
    CodeConstants.ACC_PUBLIC | CodeConstants.ACC_STATIC | CodeConstants.ACC_FINAL,
    CodeConstants.ACC_ABSTRACT, CodeConstants.ACC_PUBLIC | CodeConstants.ACC_ABSTRACT,
    CodeConstants.ACC_NATIVE | CodeConstants.ACC_PRIVATE, CodeConstants.ACC_STATIC | CodeConstants.ACC_STRICT,
    CodeConstants.ACC_SYNCHRONIZED | CodeConstants.ACC_PROTECTED,
    CodeConstants.ACC_TRANSIENT | CodeConstants.ACC_VOLATILE | CodeConstants.ACC_PUBLIC,
    CodeConstants.ACC_ENUM | CodeConstants.ACC_INTERFACE | CodeConstants.ACC_ANNOTATION,
    0xffff, -1,
  };

  static final String[] MOD_FIELDS = {
    "CLASS_ALLOWED", "CLASS_EXCLUDED", "FIELD_ALLOWED", "FIELD_EXCLUDED", "METHOD_ALLOWED", "METHOD_EXCLUDED",
  };

  // Keep in sync with CLASSES in tests/kt7_kotlin_writer.rs.
  static final String[] CLASSES = {
    "k7/Fix", "k7/Fix$Inner", "k7/Iface", "k7/Base", "k7/Annos", "k7/One", "k7/PkgOne", "k7/En",
    "k7/Sfp", "k7/Gen", "k7/Rec",
  };

  static Map<String, ClassNode> nodes = new HashMap<>();

  static ClassNode node(String name) throws Exception {
    ClassNode node = nodes.get(name);
    if (node != null) return node;
    StructClass cl = DecompilerContext.getStructContext().getClass(name);
    node = new ClassNode(ClassNode.Type.ROOT, cl);
    node.simpleName = name.substring(name.lastIndexOf('/') + 1);
    node.access = cl.getAccessFlags();
    ClassWrapper wrapper = new ClassWrapper(node);
    Field field = ClassNode.class.getDeclaredField("wrapper");
    field.setAccessible(true);
    field.set(node, wrapper);
    for (StructMethod mt : cl.getMethods()) {
      MethodDescriptor md = MethodDescriptor.parseDescriptor(mt.getDescriptor());
      MethodWrapper mw = new MethodWrapper(null, new VarProcessor(mt, md), mt, cl, new CounterContainer());
      mw.decompileError = new RuntimeException("boom");
      wrapper.getMethods().addWithKey(mw, InterpreterUtil.makeUniqueKey(mt.getName(), mt.getDescriptor()));
    }
    nodes.put(name, node);
    return node;
  }

  static void enter(ClassNode node) {
    DecompilerContext.setProperty(DecompilerContext.CURRENT_CLASS_NODE, node);
    DecompilerContext.startClass(new ImportCollector(node));
  }

  static String text(TextBuffer buffer) {
    return esc(buffer.convertToStringAndAllowDataDiscard());
  }

  static String member(StructMember mb) {
    if (mb instanceof StructMethod mt) return mt.getName() + mt.getDescriptor();
    if (mb instanceof StructField fd) return fd.getName() + ":" + fd.getDescriptor();
    return "class";
  }

  static List<StructMember> members(StructClass cl) {
    List<StructMember> list = new ArrayList<>();
    list.add(cl);
    list.addAll(cl.getFields());
    list.addAll(cl.getMethods());
    return list;
  }

  static boolean writableMethod(String cls, StructMethod mt) {
    return mt.getDescriptor().equals("()V") || cls.equals("k7/En") && mt.getName().equals("<init>");
  }

  public static void main(String[] args) throws Exception {
    File dir = new File(args[0]);

    Map<String, Object> options = new HashMap<>();
    options.put(IFernflowerPreferences.INCLUDE_JAVA_RUNTIME, "0");
    options.put(IFernflowerPreferences.DUMP_EXCEPTION_ON_ERROR, "0");
    options.put(IFernflowerPreferences.DUMP_BYTECODE_ON_ERROR, "1");
    IFernflowerLogger logger = new IFernflowerLogger() {
      @Override public void writeMessage(String message, Severity severity) { }
      @Override public void writeMessage(String message, Severity severity, Throwable t) { }
    };
    // The plugin classes are on the class path: no second copy through the plugin sources.
    PluginSources.PLUGIN_SOURCES.clear();
    Fernflower fernflower = new Fernflower(null, options, logger);
    fernflower.addSource(new File(dir, "classes"));

    for (String id : IDENTIFIERS) {
      System.out.println("ident\t" + esc(id) + "\t" + esc(KotlinWriter.toValidKotlinIdentifier(id)));
    }
    System.out.println("ident\t\\N\t" + esc(KotlinWriter.toValidKotlinIdentifier(null)));

    Method appendModifiers = KotlinWriter.class.getDeclaredMethod("appendModifiers",
      TextBuffer.class, int.class, int.class, boolean.class, int.class);
    appendModifiers.setAccessible(true);
    List<int[]> pairs = new ArrayList<>();
    for (int i = 0; i < MOD_FIELDS.length; i += 2) {
      Field allowed = KotlinWriter.class.getDeclaredField(MOD_FIELDS[i]);
      Field excluded = KotlinWriter.class.getDeclaredField(MOD_FIELDS[i + 1]);
      allowed.setAccessible(true);
      excluded.setAccessible(true);
      pairs.add(new int[] {allowed.getInt(null), excluded.getInt(null)});
    }
    pairs.add(new int[] {CodeConstants.ACC_FINAL, 0});
    pairs.add(new int[] {-1, 0});
    for (int flags : FLAG_SETS) {
      for (int[] pair : pairs) {
        for (boolean iface : new boolean[] {false, true}) {
          TextBuffer buffer = new TextBuffer();
          appendModifiers.invoke(null, buffer, flags, pair[0], iface, pair[1]);
          System.out.println("mods\t" + flags + "\t" + pair[0] + "\t" + iface + "\t" + pair[1] + "\t" + text(buffer));
        }
      }
    }


    Method isNullable = KotlinWriter.class.getDeclaredMethod("isNullable", StructMember.class);
    isNullable.setAccessible(true);
    int[] targets = {TypeAnnotation.METHOD_RETURN_TYPE, TypeAnnotation.FIELD, TypeAnnotation.CLASS_TYPE_PARAMETER, TypeAnnotation.METHOD_PARAMETER};

    for (String name : CLASSES) {
      ClassNode node = node(name);
      enter(node);
      StructClass cl = node.classStruct;
      for (StructMember mb : members(cl)) {
        for (int target : targets) {
          for (int combo = 0; combo < 4; combo++) {
            TextBuffer buffer = new TextBuffer();
            KotlinWriter.appendJvmAnnotations(buffer, 1, mb, (combo & 1) != 0, (combo & 2) != 0, cl.getPool(), target);
            System.out.println("jvm\t" + name + "\t" + member(mb) + "\t" + target + "\t" + combo + "\t" + text(buffer));
          }
        }
      }
      for (StructMethod mt : cl.getMethods()) {
        System.out.println("nullable\t" + name + "\t" + member(mt) + "\t" + isNullable.invoke(null, mt));
        MethodDescriptor md = MethodDescriptor.parseDescriptor(mt.getDescriptor());
        System.out.println("search\t" + name + "\t" + member(mt) + "\t"
          + KotlinWriter.searchForMethod(cl, mt.getName(), md, false) + "\t"
          + KotlinWriter.searchForMethod(cl, mt.getName(), md, true));
        for (int param = 0; param <= md.params.length; param++) {
          TextBuffer buffer = new TextBuffer();
          boolean nullable = KotlinWriter.processParameterAnnotations(buffer, mt, param);
          System.out.println("param\t" + name + "\t" + member(mt) + "\t" + param + "\t" + nullable + "\t" + text(buffer));
        }
      }
    }

    // appendAnnotations: only members whose declaration annotations are all filtered.
    {
      ClassNode node = node("k7/Annos");
      enter(node);
      for (StructMember mb : members(node.classStruct)) {
        int target = mb instanceof StructMethod ? TypeAnnotation.METHOD_RETURN_TYPE : mb instanceof StructField ? TypeAnnotation.FIELD : -1;
        for (int indent : new int[] {-1, 0, 2}) {
          TextBuffer buffer = new TextBuffer();
          KotlinWriter.appendAnnotations(buffer, indent, mb, target);
          System.out.println("annos\t" + member(mb) + "\t" + indent + "\t" + text(buffer));
        }
      }
    }

    // hideConstructor
    for (String name : new String[] {"k7/One", "k7/PkgOne", "k7/Fix", "k7/En"}) {
      ClassNode node = node(name);
      enter(node);
      for (String hdc : new String[] {"1", "0"}) {
        DecompilerContext.setProperty(IFernflowerPreferences.HIDE_DEFAULT_CONSTRUCTOR, hdc);
        for (int combo = 0; combo < 8; combo++) {
          for (int flags : new int[] {0, CodeConstants.ACC_PUBLIC, CodeConstants.ACC_PRIVATE, CodeConstants.ACC_PROTECTED, CodeConstants.ACC_PUBLIC | CodeConstants.ACC_STATIC}) {
            boolean init = (combo & 1) == 0;
            boolean thr = (combo & 2) != 0;
            int params = (combo & 4) != 0 ? 1 : 0;
            System.out.println("hide\t" + name + "\t" + hdc + "\t" + init + "\t" + thr + "\t" + params + "\t" + flags + "\t"
              + KotlinWriter.hideConstructor(node, init, thr, params, flags));
          }
        }
      }
      DecompilerContext.setProperty(IFernflowerPreferences.HIDE_DEFAULT_CONSTRUCTOR, "1");
    }

    // appendTypeParameters
    {
      ClassNode node = node("k7/Gen");
      enter(node);
      GenericClassDescriptor sig = node.classStruct.getSignature();
      TextBuffer buffer = new TextBuffer();
      KotlinWriter.appendTypeParameters(buffer, sig.fparameters, sig.fbounds);
      System.out.println("typeparams\tclass\t" + text(buffer));
      for (StructMethod mt : node.classStruct.getMethods()) {
        GenericMethodDescriptor msig = mt.getSignature();
        if (msig == null || msig.typeParameters.isEmpty()) continue;
        buffer = new TextBuffer();
        KotlinWriter.appendTypeParameters(buffer, msig.typeParameters, msig.typeParameterBounds);
        System.out.println("typeparams\t" + member(mt) + "\t" + text(buffer));
      }
    }

    KotlinWriter writer = new KotlinWriter();

    // writeField
    for (String name : new String[] {"k7/Fix", "k7/Fix$Inner", "k7/Iface", "k7/En", "k7/Annos", "k7/Rec"}) {
      ClassNode node = node(name);
      enter(node);
      for (StructField fd : node.classStruct.getFields()) {
        TextBuffer buffer = new TextBuffer();
        writer.writeField(node.getWrapper(), node.classStruct, fd, buffer, 1);
        System.out.println("field\t" + name + "\t" + member(fd) + "\t" + text(buffer));
      }
    }

    // writeMethod on failed methods
    for (String name : CLASSES) {
      ClassNode node = node(name);
      enter(node);
      List<StructMethod> methods = node.classStruct.getMethods();
      for (int i = 0; i < methods.size(); i++) {
        StructMethod mt = methods.get(i);
        if (!writableMethod(name, mt)) continue;
        TextBuffer buffer = new TextBuffer();
        boolean shown = writer.writeMethod(node, mt, i, buffer, 1);
        System.out.println("method\t" + name + "\t" + member(mt) + "\t" + shown + "\t" + text(buffer));
      }
    }
  }
}
