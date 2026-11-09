import java.io.*;
import java.lang.reflect.Field;
import java.util.*;
import java.util.function.Function;
import java.util.jar.Manifest;

import org.jetbrains.java.decompiler.code.CodeConstants;
import org.jetbrains.java.decompiler.main.ClassesProcessor;
import org.jetbrains.java.decompiler.main.ClassesProcessor.ClassNode;
import org.jetbrains.java.decompiler.main.DecompilerContext;
import org.jetbrains.java.decompiler.main.Fernflower;
import org.jetbrains.java.decompiler.main.extern.IFernflowerLogger;
import org.jetbrains.java.decompiler.main.extern.IResultSaver;
import org.jetbrains.java.decompiler.main.plugins.PluginSources;
import org.jetbrains.java.decompiler.main.rels.ClassWrapper;
import org.jetbrains.java.decompiler.main.rels.MethodWrapper;
import org.jetbrains.java.decompiler.modules.decompiler.exps.AnnotationExprent;
import org.jetbrains.java.decompiler.modules.decompiler.exps.Exprent;
import org.jetbrains.java.decompiler.struct.StructClass;
import org.jetbrains.java.decompiler.struct.StructField;
import org.jetbrains.java.decompiler.struct.StructMember;
import org.jetbrains.java.decompiler.struct.StructMethod;
import org.jetbrains.java.decompiler.struct.attr.StructGeneralAttribute;
import org.jetbrains.java.decompiler.util.TextBuffer;
import org.vineflower.kotlin.KotlinChooser;
import org.vineflower.kotlin.struct.*;
import org.vineflower.kotlin.util.KEnumProcessor;
import org.vineflower.kotlin.util.KotlinObjectProcessors;
import org.vineflower.kt.metadata.ProtoBuf;
import org.vineflower.kt.metadata.deserialization.Flags;

/**
 * Oracle for the kuna-jvm KT4 slice: runs the Kotlin plugin's own member
 * parsers (through KotlinChooser.parseMetadataFor) over the classes of a
 * directory, then the Java pipeline (no plugins) over each root class, and
 * dumps what KFunction/KProperty/KConstructor/DefaultArgsMap and the object
 * and enum processors record.
 *
 *   members <classes-dir>             parse facts, object/enum processors, accessor wiring,
 *                                     default arguments (needs KType, KContract, searchForMethod)
 *   ctors <classes-dir> <class>...    KConstructor.parse + writePrimaryConstructor/stringify of
 *                                     classes whose constructors take no parameters
 *   isolated <classes-dir>            KClass attributes only: KEnumProcessor, markObjectInstance,
 *                                     DefaultArgsMap.from over every default-argument bridge
 */
public class KtMembersDump {
  static PrintStream out;
  static final List<String> logs = new ArrayList<>();

  public static void main(String[] args) throws Exception {
    out = new PrintStream(new FileOutputStream(FileDescriptor.out), false, "UTF-8");
    PluginSources.PLUGIN_SOURCES.clear();
    Map<String, Object> options = new HashMap<>();
    options.put("include-runtime", "0");
    options.put("log-level", "warn");
    options.put("kt-unknown-defaults", "...");
    options.put("kt-show-public", "0");
    Fernflower ff = new Fernflower(new NullSaver(), options, new Logger());
    ff.addSource(new File(args[1]));
    switch (args[0]) {
      case "members" -> members();
      case "ctors" -> ctors(Arrays.copyOfRange(args, 2, args.length));
      case "isolated" -> isolated();
      default -> throw new IllegalArgumentException(args[0]);
    }
    out.flush();
  }

  // ---- text

  static String esc(String s) {
    if (s == null) return "null";
    StringBuilder b = new StringBuilder();
    for (int i = 0; i < s.length(); i++) {
      char c = s.charAt(i);
      if (c < 0x20 || c >= 0x7f || c == '\\') {
        b.append(String.format("\\u%04x", (int) c));
      } else {
        b.append(c);
      }
    }
    return b.toString();
  }

  static String exc(Throwable t) {
    return "!" + t.getClass().getSimpleName();
  }

  static void p(String s) { out.println(s); }

  static void flushLogs() {
    for (String l : logs) p("log\t" + esc(l));
    logs.clear();
  }

  static String type(KType t) {
    if (t == null) return "-";
    String s = t.kotlinType + (t.isNullable ? "?" : "") + "|" + t;
    if (t.typeParameterName != null) s += "|T=" + t.typeParameterName;
    if (t.typeAliasName != null) s += "|A=" + t.typeAliasName;
    s += "|args=" + (t.typeArguments == null ? "-" : String.valueOf(t.typeArguments.length));
    return s;
  }

  static String param(KParameter p) {
    return p.name() + ":" + p.flags() + ":" + type(p.type()) + ":" + type(p.varargType()) + ":" + p.typeId();
  }

  static String params(KParameter[] ps) {
    StringJoiner j = new StringJoiner(", ", "[", "]");
    for (KParameter p : ps) j.add(param(p));
    return j.toString();
  }

  static String tparam(KTypeParameter tp) {
    StringJoiner j = new StringJoiner(" & ");
    for (KType b : tp.upperBounds()) j.add(type(b));
    return tp.name() + ":" + tp.reified() + ":" + tp.variance() + ":" + tp.id() + ":<" + j + ">";
  }

  static String member(StructMember m) {
    if (m == null) return "-";
    if (m instanceof StructMethod mt) return mt.getName() + " " + mt.getDescriptor();
    if (m instanceof StructField fd) return fd.getName() + " " + fd.getDescriptor();
    return ((StructClass) m).qualifiedName;
  }

  static String ctor(KConstructor c) {
    return "Constructor flags=" + c.flags() + " primary=" + c.isPrimary() + " classFlags=" + c.classFlags()
      + " class=" + c.classStruct().qualifiedName + " method=" + member(c.methodStruct())
      + " supplier=" + (c.methodSupplier() != null) + " params=" + params(c.parameters());
  }

  static String describe(KElement e) {
    if (e == null) return null;
    if (e instanceof KHiddenElement h) return "Hidden " + h.name();
    if (e instanceof KClass c) {
      String companion = c.proto().hasCompanionObjectName() ? c.resolver().resolve(c.proto().getCompanionObjectName()) : "-";
      return "Class flags=" + c.flags() + " companion=" + companion;
    }
    if (e instanceof KFile f) return "File multifile=" + f.multifileName();
    if (e instanceof KMultifileFacade m) return "Facade " + m.files().size();
    if (e instanceof KFunctionReference) return "FunctionReference";
    if (e instanceof KFunction f) {
      if (f == KFunction.FAILED_LAMBDA) return "FailedLambda";
      StringJoiner ctx = new StringJoiner(", ", "[", "]");
      for (KType t : f.contextReceiverTypes()) ctx.add(type(t));
      StringJoiner tps = new StringJoiner(", ", "[", "]");
      for (KTypeParameter tp : f.typeParameters()) tps.add(tparam(tp));
      return "Function name=" + f.name() + " flags=" + f.flags() + " method=" + member(f.methodStruct())
        + " class=" + f.classStruct().qualifiedName + " override=" + f.knownOverride()
        + " receiver=" + type(f.receiverType()) + " return=" + type(f.returnType()) + " ctx=" + ctx
        + " tparams=" + tps + " contract=" + (f.contract() == null ? "-" : String.valueOf(f.contract().effects.size()))
        + " params=" + params(f.parameters());
    }
    if (e instanceof KProperty p) {
      return "Property name=" + p.name() + " flags=" + p.flags() + " type=" + type(p.type())
        + " getter=" + (p.getter() == null ? "-" : String.valueOf(p.getter().flags()))
        + " setter=" + (p.setter() == null ? "-" : String.valueOf(p.setter().flags()))
        + " setterParam=" + p.setterParamName() + " field=" + member(p.underlyingField())
        + " initializer=" + (p.initializer() != null) + " annotations=" + annotations(p) + " class=" + p.classStruct().qualifiedName;
    }
    if (e instanceof KConstructor c) return ctor(c);
    return "?" + e.getClass().getSimpleName();
  }

  static AnnotationExprent deprecatedAnnotation() throws Exception {
    Field f = KProperty.class.getDeclaredField("DEPRECATED_ANNOTATION");
    f.setAccessible(true);
    return (AnnotationExprent) f.get(null);
  }

  static String annotations(KProperty p) {
    if (p.annotations() == null) return "null";
    StringJoiner j = new StringJoiner(",", "[", "]");
    try {
      AnnotationExprent dep = deprecatedAnnotation();
      for (AnnotationExprent a : p.annotations()) j.add(a == dep ? "DEPRECATED" : a.getClassName());
    } catch (Exception ex) {
      throw new RuntimeException(ex);
    }
    return j.toString();
  }

  static List<StructClass> sortedClasses() {
    List<StructClass> classes = new ArrayList<>(DecompilerContext.getStructContext().getOwnClasses());
    classes.sort(Comparator.comparing(c -> c.qualifiedName));
    return classes;
  }

  static void dumpAttrs(StructClass cl) {
    p("class\t" + cl.qualifiedName);
    String d = describe(cl.getAttribute(KElement.KEY));
    if (d != null) p("  self\t" + d);
    KConstructor primary = cl.getAttribute(KConstructor.PRIMARY_CONSTRUCTOR);
    if (primary != null) {
      boolean same = primary.methodStruct() != null && primary.methodStruct().getAttribute(KElement.KEY) == primary;
      p("  primary\t" + ctor(primary) + " same=" + same);
    }
    for (StructField fd : cl.getFields()) {
      d = describe(fd.getAttribute(KElement.KEY));
      if (d != null) p("  field " + member(fd) + "\t" + d);
    }
    for (StructMethod mt : cl.getMethods()) {
      d = describe(mt.getAttribute(KElement.KEY));
      KProperty.PropertyMethod pm = mt.getAttribute(KProperty.PropertyMethod.KEY);
      if (d != null || pm != null) p("  method " + member(mt) + "\t" + d + (pm == null ? "" : "\t" + pm.name()));
    }
  }

  static List<ClassNode> tree(ClassNode root) {
    List<ClassNode> nodes = new ArrayList<>();
    collect(root, nodes);
    return nodes;
  }

  static void collect(ClassNode node, List<ClassNode> nodes) {
    if (node.type == ClassNode.Type.LAMBDA) return;
    nodes.add(node);
    for (ClassNode n : node.nested) collect(n, nodes);
  }

  static void enterClass(ClassNode node) {
    ClassWrapper wrapper = node.getWrapper();
    DecompilerContext.setProperty(DecompilerContext.CURRENT_CLASS, node.classStruct);
    DecompilerContext.setProperty(DecompilerContext.CURRENT_CLASS_WRAPPER, wrapper);
    DecompilerContext.setProperty(DecompilerContext.CURRENT_CLASS_NODE, node);
  }

  static String text(TextBuffer buf) {
    return esc(buf.convertToStringAndAllowDataDiscard());
  }

  static String wrapperName(MethodWrapper mw) {
    return mw == null ? "null" : member(mw.methodStruct);
  }

  static void members() throws Exception {
    List<StructClass> classes = sortedClasses();
    for (StructClass cl : classes) KotlinChooser.parseMetadataFor(cl);
    p("== parse");
    flushLogs();
    for (StructClass cl : classes) dumpAttrs(cl);

    ClassesProcessor processor = DecompilerContext.getClassProcessor();
    processor.loadClasses(null);
    for (StructClass cl : classes) {
      ClassNode root = processor.getMapRootClasses().get(cl.qualifiedName);
      if (root == null || root.type != ClassNode.Type.ROOT) continue;
      p("== root " + cl.qualifiedName);
      processor.processClass(cl);
      flushLogs();
      List<ClassNode> nodes = tree(root);
      for (ClassNode node : nodes) {
        enterClass(node);
        ClassWrapper wrapper = node.getWrapper();
        StructClass ncl = node.classStruct;
        p("node\t" + ncl.qualifiedName);
        try {
          invokeProcessors(node, true);
        } catch (Throwable t) {
          p("  processors\t" + exc(t));
        }
        hidden(wrapper);
        flushLogs();
      }
      for (ClassNode node : nodes) dumpAttrs(node.classStruct);
      for (ClassNode node : nodes) {
        enterClass(node);
        dumpWiring(node);
      }
      flushLogs();
    }
  }

  /** KotlinWriter.invokeProcessors without the class-reference pass (no 1.4 classes here). */
  static void invokeProcessors(ClassNode node, boolean companion) {
    ClassWrapper wrapper = node.getWrapper();
    StructClass cl = wrapper.getClassStruct();
    org.jetbrains.java.decompiler.modules.decompiler.InitializerProcessor.extractInitializers(wrapper);
    org.jetbrains.java.decompiler.modules.decompiler.InitializerProcessor.hideInitalizers(wrapper);
    if (cl.hasModifier(CodeConstants.ACC_ENUM) && DecompilerContext.getOption(org.jetbrains.java.decompiler.main.extern.IFernflowerPreferences.DECOMPILE_ENUM)) {
      KEnumProcessor.clearEnum(wrapper);
    }
    if (companion) KotlinObjectProcessors.parseWithCompanion(node);
    KotlinObjectProcessors.markObjectInstance(wrapper);
  }

  static void hidden(ClassWrapper wrapper) {
    List<String> hidden = new ArrayList<>(wrapper.getHiddenMembers());
    Collections.sort(hidden);
    if (!hidden.isEmpty()) p("  hidden\t" + esc(String.join(",", hidden)));
  }

  static void dumpWiring(ClassNode node) {
    ClassWrapper wrapper = node.getWrapper();
    StructClass cl = node.classStruct;
    p("wiring\t" + cl.qualifiedName);
    KConstructor primary = cl.getAttribute(KConstructor.PRIMARY_CONSTRUCTOR);
    if (primary != null && primary.methodSupplier() == null) {
      try {
        DefaultArgsMap map = primary.defaultArgsSupplier().apply(wrapper);
        StringJoiner j = new StringJoiner(",", "[", "]");
        for (KParameter p : primary.parameters()) {
          j.add(p.name() + "=" + !map.toJava(p, 1).convertToStringAndAllowDataDiscard().equals(" = ..."));
        }
        p("  annotation-defaults\t" + j);
      } catch (Throwable t) {
        p("  annotation-defaults\t" + exc(t));
      }
    }
    for (StructMethod mt : cl.getMethods()) {
      KElement e = mt.getAttribute(KElement.KEY);
      KParameter[] ps;
      Function<ClassWrapper, MethodWrapper> supplier;
      Function<ClassWrapper, DefaultArgsMap> defaults;
      if (e instanceof KFunction f) {
        ps = f.parameters();
        supplier = f.methodSupplier();
        defaults = f.defaultArgsSupplier();
      } else if (e instanceof KConstructor c) {
        ps = c.parameters();
        supplier = c.methodSupplier();
        defaults = c.defaultArgsSupplier();
      } else {
        if (e instanceof KProperty prop && mt.getAttribute(KProperty.PropertyMethod.KEY) == KProperty.PropertyMethod.GETTER) {
          dumpProperty(wrapper, prop);
        }
        continue;
      }
      String key = member(mt);
      MethodWrapper calling;
      try {
        calling = supplier.apply(wrapper);
        p("  supplier " + key + "\t" + wrapperName(calling));
      } catch (Throwable t) {
        p("  supplier " + key + "\t" + exc(t));
        continue;
      }
      boolean any = false;
      for (KParameter p : ps) any |= Flags.DECLARES_DEFAULT_VALUE.get(p.flags());
      if (!any) continue;
      DecompilerContext.setProperty(DecompilerContext.CURRENT_METHOD_WRAPPER, calling);
      try {
        DefaultArgsMap map = defaults.apply(wrapper);
        p("  defaults " + key + "\t" + member(map.getDefaultMethod()));
        for (KParameter p : ps) {
          String v;
          try {
            v = text(map.toJava(p, 1));
          } catch (Throwable t) {
            v = exc(t);
          }
          p("    " + p.name() + "\t" + Flags.DECLARES_DEFAULT_VALUE.get(p.flags()) + "\t" + v);
        }
      } catch (Throwable t) {
        p("  defaults " + key + "\t" + exc(t));
      }
      DecompilerContext.setProperty(DecompilerContext.CURRENT_METHOD_WRAPPER, null);
    }
  }

  static void dumpProperty(ClassWrapper wrapper, KProperty prop) {
    String g = prop.getter() == null ? "-" : wrapperName(prop.getter().methodSupplier().apply(wrapper));
    String s = prop.setter() == null ? "-" : wrapperName(prop.setter().methodSupplier().apply(wrapper));
    String i;
    if (prop.initializer() == null) {
      i = "-";
    } else if (prop.underlyingField() != null && prop.underlyingField().hasAttribute(StructGeneralAttribute.ATTRIBUTE_CONSTANT_VALUE)
      && !Flags.IS_DELEGATED.get(prop.flags())) {
      i = "const";
    } else {
      try {
        Exprent ex = prop.initializer().apply(wrapper);
        i = ex == null ? "null" : text(ex.toJava(1));
      } catch (Throwable t) {
        i = exc(t);
      }
    }
    p("  property " + prop.name() + "\tget=" + g + "\tset=" + s + "\tinit=" + i);
  }

  static void ctors(String[] names) throws Exception {
    AnnotationExprent dep = deprecatedAnnotation();
    ClassesProcessor processor = DecompilerContext.getClassProcessor();
    processor.loadClasses(null);
    for (String name : names) {
      StructClass cl = DecompilerContext.getStructContext().getClass(name);
      p("== ctors " + name);
      ProtoBuf.Class proto = KtProto.classProto(cl);
      KConstructor.parse(cl, proto, KtProto.resolver(cl));
      flushLogs();
      dumpAttrs(cl);
      ClassNode root = processor.getMapRootClasses().get(cl.qualifiedName);
      processor.processClass(cl);
      flushLogs();
      enterClass(root);
      ClassWrapper wrapper = root.getWrapper();
      p("deprecated\t" + text(dep.toJava(0)));
      for (StructMethod mt : cl.getMethods()) {
        if (!(mt.getAttribute(KElement.KEY) instanceof KConstructor c)) continue;
        MethodWrapper mw = c.methodSupplier().apply(wrapper);
        DecompilerContext.setProperty(DecompilerContext.CURRENT_METHOD_WRAPPER, mw);
        if (c.isPrimary()) {
          TextBuffer header = new TextBuffer();
          String h;
          try {
            h = String.valueOf(c.writePrimaryConstructor(wrapper, header, 1));
          } catch (Throwable t) {
            h = exc(t);
          }
          p("primary " + member(mt) + "\t" + h + "\t" + text(header));
        }
        TextBuffer buf = new TextBuffer();
        String r;
        try {
          r = String.valueOf(c.stringify(wrapper, buf, 1));
        } catch (Throwable t) {
          r = exc(t);
        }
        p("stringify " + member(mt) + "\t" + r + "\t" + text(buf));
        flushLogs();
      }
    }
  }

  /** Needs none of KType/KTypeParameter/KContract: KClass attributes only, synthetic parameters. */
  static void isolated() throws Exception {
    List<StructClass> classes = sortedClasses();
    for (StructClass cl : classes) {
      if (KtProto.kind(cl) == 1) cl.getAttributes().put(KElement.KEY, new KClass(KtProto.classProto(cl), KtProto.resolver(cl)));
    }
    ClassesProcessor processor = DecompilerContext.getClassProcessor();
    processor.loadClasses(null);
    for (StructClass cl : classes) {
      ClassNode root = processor.getMapRootClasses().get(cl.qualifiedName);
      if (root == null || root.type != ClassNode.Type.ROOT) continue;
      p("== root " + cl.qualifiedName);
      processor.processClass(cl);
      flushLogs();
      for (ClassNode node : tree(root)) {
        enterClass(node);
        ClassWrapper wrapper = node.getWrapper();
        StructClass ncl = node.classStruct;
        p("node\t" + ncl.qualifiedName);
        invokeProcessors(node, false);
        hidden(wrapper);
        for (StructField fd : ncl.getFields()) {
          String d = describe(fd.getAttribute(KElement.KEY));
          if (d != null) p("  field " + member(fd) + "\t" + d);
        }
        for (StructMethod dm : ncl.getMethods()) {
          if (!isDefaultBridge(dm)) continue;
          StructMethod caller = null;
          for (StructMethod mt : ncl.getMethods()) {
            if (mt != dm && bridges(ncl, dm, mt)) {
              caller = mt;
              break;
            }
          }
          if (caller == null) {
            p("  bridge " + member(dm) + "\t-");
            continue;
          }
          int count = caller.methodDescriptor().params.length;
          KParameter[] ps = new KParameter[count];
          for (int i = 0; i < count; i++) ps[i] = new KParameter(2, "p" + i, KType.NOTHING, null, 0);
          MethodWrapper calling = wrapper.getMethodWrapper(caller.getName(), caller.getDescriptor());
          DecompilerContext.setProperty(DecompilerContext.CURRENT_METHOD_WRAPPER, calling);
          try {
            DefaultArgsMap map = DefaultArgsMap.from(wrapper.getMethodWrapper(dm.getName(), dm.getDescriptor()), calling, ps);
            p("  bridge " + member(dm) + "\t" + member(caller) + "\t" + member(map.getDefaultMethod()));
            for (KParameter p : ps) {
              String v;
              try {
                v = text(map.toJava(p, 1));
              } catch (Throwable t) {
                v = exc(t);
              }
              p("    " + p.name() + "\t" + v);
            }
          } catch (Throwable t) {
            p("  bridge " + member(dm) + "\t" + member(caller) + "\t" + exc(t));
          }
          DecompilerContext.setProperty(DecompilerContext.CURRENT_METHOD_WRAPPER, null);
        }
        flushLogs();
      }
    }
  }

  static final String MARKER = "Lkotlin/jvm/internal/DefaultConstructorMarker;";

  static boolean isDefaultBridge(StructMethod mt) {
    if (mt.getName().endsWith("$default")) return true;
    return mt.getName().equals("<init>") && mt.getDescriptor().endsWith("I" + MARKER + ")V");
  }

  static boolean bridges(StructClass cl, StructMethod bridge, StructMethod mt) {
    String d = mt.getDescriptor();
    String params = d.substring(1, d.indexOf(')'));
    String ret = d.substring(d.indexOf(')') + 1);
    for (int k = 1; k <= 3; k++) {
      String masks = "I".repeat(k);
      String expected;
      if (bridge.getName().equals("<init>")) {
        if (!mt.getName().equals("<init>")) return false;
        expected = "(" + params + masks + MARKER + ")V";
      } else {
        if (!bridge.getName().equals(mt.getName() + "$default")) return false;
        String self = mt.hasModifier(CodeConstants.ACC_STATIC) ? "" : "L" + cl.qualifiedName + ";";
        expected = "(" + self + params + masks + "Ljava/lang/Object;)" + ret;
      }
      if (expected.equals(bridge.getDescriptor())) return true;
    }
    return false;
  }

  static final class KtProto {
    static org.vineflower.kt.protobuf.ExtensionRegistryLite EXT = org.vineflower.kt.protobuf.ExtensionRegistryLite.newInstance();
    static { org.vineflower.kt.metadata.jvm.JvmProtoBuf.registerAllExtensions(EXT); }

    static Map<StructClass, Object[]> cache = new HashMap<>();

    static Object[] read(StructClass cl) throws Exception {
      if (cache.containsKey(cl)) return cache.get(cl);
      org.jetbrains.java.decompiler.struct.attr.StructAnnotationAttribute attr =
        cl.getAttribute(StructGeneralAttribute.ATTRIBUTE_RUNTIME_VISIBLE_ANNOTATIONS);
      AnnotationExprent anno = null;
      if (attr != null) {
        for (AnnotationExprent a : attr.getAnnotations()) if (a.getClassName().equals("kotlin/Metadata")) anno = a;
      }
      if (anno == null || anno.getParNames().indexOf("d1") == -1 || anno.getParNames().indexOf("d2") == -1) {
        Object[] r = {null, null, 0};
        cache.put(cl, r);
        return r;
      }
      int k = (Integer) ((org.jetbrains.java.decompiler.modules.decompiler.exps.ConstExprent) anno.getParValues().get(anno.getParNames().indexOf("k"))).getValue();
      String[] d1 = strings(anno.getParValues().get(anno.getParNames().indexOf("d1")));
      String[] d2 = strings(anno.getParValues().get(anno.getParNames().indexOf("d2")));
      ByteArrayInputStream input = new ByteArrayInputStream(org.vineflower.kotlin.metadata.BitEncoding.decodeBytes(d1));
      org.vineflower.kt.metadata.jvm.JvmProtoBuf.StringTableTypes types =
        org.vineflower.kt.metadata.jvm.JvmProtoBuf.StringTableTypes.parseDelimitedFrom(input, EXT);
      org.vineflower.kotlin.metadata.MetadataNameResolver resolver = new org.vineflower.kotlin.metadata.MetadataNameResolver(types, d2);
      ProtoBuf.Class pcl = k == 1 ? ProtoBuf.Class.parseFrom(input, EXT) : null;
      Object[] r = {pcl, resolver, k};
      cache.put(cl, r);
      return r;
    }

    static int kind(StructClass cl) throws Exception { return (Integer) read(cl)[2]; }

    static String[] strings(Exprent e) {
      return ((org.jetbrains.java.decompiler.modules.decompiler.exps.NewExprent) e).getLstArrayElements().stream()
        .map(x -> (String) ((org.jetbrains.java.decompiler.modules.decompiler.exps.ConstExprent) x).getValue())
        .toArray(String[]::new);
    }

    static ProtoBuf.Class classProto(StructClass cl) throws Exception { return (ProtoBuf.Class) read(cl)[0]; }

    static org.vineflower.kotlin.metadata.MetadataNameResolver resolver(StructClass cl) throws Exception {
      return (org.vineflower.kotlin.metadata.MetadataNameResolver) read(cl)[1];
    }
  }

  static final class Logger extends IFernflowerLogger {
    @Override
    public void writeMessage(String message, Severity severity) {
      if (accepts(severity)) logs.add(severity + " " + message);
    }

    @Override
    public void writeMessage(String message, Severity severity, Throwable t) {
      if (accepts(severity)) logs.add(severity + " " + message);
    }
  }

  static final class NullSaver implements IResultSaver {
    public void saveFolder(String path) { }
    public void copyFile(String source, String path, String entryName) { }
    public void saveClassFile(String path, String qualifiedName, String entryName, String content, int[] mapping) { }
    public void createArchive(String path, String archiveName, Manifest manifest) { }
    public void saveDirEntry(String path, String archiveName, String entryName) { }
    public void copyEntry(String source, String path, String archiveName, String entry) { }
    public void saveClassEntry(String path, String archiveName, String qualifiedName, String entryName, String content) { }
    public void closeArchive(String path, String archiveName) { }
  }
}
