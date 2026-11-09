import java.io.*;
import java.lang.reflect.Field;
import java.nio.file.*;
import java.util.*;
import java.util.stream.*;
import java.util.zip.*;

import org.jetbrains.java.decompiler.main.ClassesProcessor;
import org.jetbrains.java.decompiler.main.DecompilerContext;
import org.jetbrains.java.decompiler.main.collectors.ImportCollector;
import org.jetbrains.java.decompiler.main.extern.IFernflowerLogger;
import org.jetbrains.java.decompiler.main.extern.IFernflowerPreferences;
import org.jetbrains.java.decompiler.modules.decompiler.exps.AnnotationExprent;
import org.jetbrains.java.decompiler.modules.decompiler.exps.ConstExprent;
import org.jetbrains.java.decompiler.modules.decompiler.exps.Exprent;
import org.jetbrains.java.decompiler.modules.decompiler.exps.NewExprent;
import org.jetbrains.java.decompiler.struct.StructClass;
import org.jetbrains.java.decompiler.struct.StructContext;
import org.jetbrains.java.decompiler.struct.StructField;
import org.jetbrains.java.decompiler.struct.StructMethod;
import org.jetbrains.java.decompiler.struct.attr.StructAnnotationAttribute;
import org.jetbrains.java.decompiler.struct.attr.StructGeneralAttribute;
import org.jetbrains.java.decompiler.struct.attr.StructGenericSignatureAttribute;
import org.jetbrains.java.decompiler.struct.gen.VarType;
import org.jetbrains.java.decompiler.struct.gen.generics.GenericType;
import org.jetbrains.java.decompiler.util.Key;
import org.jetbrains.java.decompiler.util.TextBuffer;
import org.vineflower.kotlin.KotlinChooser;
import org.vineflower.kotlin.KotlinImportCollector;
import org.vineflower.kotlin.metadata.BitEncoding;
import org.vineflower.kotlin.metadata.MetadataNameResolver;
import org.vineflower.kotlin.struct.*;
import org.vineflower.kotlin.util.KTypes;
import org.vineflower.kotlin.util.KUtils;
import org.vineflower.kt.metadata.ProtoBuf;
import org.vineflower.kt.metadata.jvm.JvmProtoBuf;
import org.vineflower.kt.protobuf.ExtensionRegistryLite;

/**
 * Oracle for the kuna-jvm KT3 slice: runs the Kotlin plugin's own KType,
 * KTypeParameter, KParameter, KContract, KTypes, KUtils, KotlinImportCollector
 * and KotlinChooser on real metadata and prints what they produce.
 *
 *   meta <label> <jar|dir> [<id-list>]   per class: the raw k/d1/d2/xs, then the element dump
 *   metaraw <file>                        the same for crafted k/d1/d2/xs (gen_crafted.py)
 *   chooser <label> <dir>                 isLanguage twice per class, under three option sets
 *   kotlintypes <label> <jar|dir> [<n>]   KTypes.getKotlinType over the type signatures of the
 *                                         classes (every n-th), plus crafted ones
 *   misc                                  getJavaSignature and appendVisibility on crafted input
 */
public class KtElementDump {
  static final ExtensionRegistryLite EXT = ExtensionRegistryLite.newInstance();
  static { JvmProtoBuf.registerAllExtensions(EXT); }

  static PrintStream out;

  public static void main(String[] args) throws Exception {
    out = new PrintStream(new FileOutputStream(FileDescriptor.out), false, "UTF-8");
    StructGeneralAttribute.init();
    switch (args[0]) {
      case "meta" -> meta(args[1], args[2], args.length > 3 ? args[3] : null);
      case "metaraw" -> metaRaw(args[1]);
      case "chooser" -> chooser(args[1], args[2]);
      case "kotlintypes" -> kotlinTypes(args[1], args[2], args.length > 3 ? Integer.parseInt(args[3]) : 1);
      case "misc" -> misc();
      default -> throw new IllegalArgumentException(args[0]);
    }
    out.flush();
  }

  // ---- text

  static String esc(String s) {
    if (s == null) return "\\null";
    StringBuilder b = new StringBuilder();
    for (int i = 0; i < s.length(); i++) {
      char c = s.charAt(i);
      if (c <= 0x20 || c >= 0x7f || c == '\\') {
        b.append(String.format("\\u%04x", (int) c));
      } else {
        b.append(c);
      }
    }
    return b.toString();
  }

  static void p(String s) { out.println(s); }

  /** A d1 string: `h` and two hex digits per char when every char fits a byte, else escaped. */
  static String bytes(String s) {
    StringBuilder b = new StringBuilder("h ");
    for (int i = 0; i < s.length(); i++) {
      char c = s.charAt(i);
      if (c > 0xff) return "s " + esc(s);
      b.append(String.format("%02x", (int) c));
    }
    return b.toString();
  }

  static String exc(Throwable t) { return "!" + t.getClass().getSimpleName(); }

  static void item(Runnable r) {
    try {
      r.run();
    } catch (Throwable t) {
      p(exc(t));
    }
  }

  // ---- context

  static final class Log extends IFernflowerLogger {
    final List<String> lines = new ArrayList<>();

    Log() { setSeverity(Severity.WARN); }

    @Override
    public void writeMessage(String message, Severity severity) {
      if (accepts(severity)) lines.add(severity.name() + " " + esc(message));
    }

    @Override
    public void writeMessage(String message, Severity severity, Throwable t) {
      if (accepts(severity)) lines.add(severity.name() + " " + esc(message) + " " + exc(t));
    }
  }

  static StructContext install(Log log, String... options) {
    Map<String, Object> props = new HashMap<>(IFernflowerPreferences.DEFAULTS);
    for (int i = 0; i < options.length; i += 2) props.put(options[i], options[i + 1]);
    StructContext sc = new StructContext(null, null);
    ClassesProcessor cp = new ClassesProcessor(sc);
    DecompilerContext.setCurrentContext(new DecompilerContext(props, log, sc, cp, null));
    return sc;
  }

  static void set(Object o, String name, Object value) throws Exception {
    Field f = ImportCollector.class.getDeclaredField(name);
    f.setAccessible(true);
    f.set(o, value);
  }

  /** An ImportCollector of an empty class in package `pkg` (slashes). */
  static ImportCollector emptyCollector(String pkg) {
    try {
      Field uf = sun.misc.Unsafe.class.getDeclaredField("theUnsafe");
      uf.setAccessible(true);
      sun.misc.Unsafe u = (sun.misc.Unsafe) uf.get(null);
      ImportCollector ic = (ImportCollector) u.allocateInstance(ImportCollector.class);
      set(ic, "mapSimpleNames", new HashMap<String, String>());
      set(ic, "setNotImportedNames", new HashSet<String>());
      set(ic, "setFieldNames", new HashSet<String>());
      set(ic, "mapInnerClassNames", new HashMap<String, Map<String, String>>());
      set(ic, "currentPackageSlash", pkg.isEmpty() ? "" : pkg + "/");
      set(ic, "currentPackagePoint", pkg.replace('/', '.'));
      set(ic, "writeLocked", false);
      return ic;
    } catch (Exception e) {
      throw new RuntimeException(e);
    }
  }

  static KotlinImportCollector freshCollector(String pkg) {
    KotlinImportCollector kc = new KotlinImportCollector(emptyCollector(pkg));
    DecompilerContext.setImportCollector(kc);
    return kc;
  }

  static List<String> classNames(String path) throws IOException {
    List<String> names = new ArrayList<>();
    File f = new File(path);
    if (f.isDirectory()) {
      try (Stream<Path> s = Files.walk(f.toPath())) {
        s.filter(x -> x.toString().endsWith(".class"))
          .forEach(x -> {
            String rel = f.toPath().relativize(x).toString().replace(File.separatorChar, '/');
            names.add(rel.substring(0, rel.length() - 6));
          });
      }
    } else {
      try (ZipFile z = new ZipFile(f)) {
        for (Enumeration<? extends ZipEntry> e = z.entries(); e.hasMoreElements(); ) {
          String n = e.nextElement().getName();
          if (n.endsWith(".class") && !n.startsWith("META-INF/") && !n.endsWith("module-info.class")) {
            names.add(n.substring(0, n.length() - 6));
          }
        }
      }
    }
    Collections.sort(names);
    return names;
  }

  // ---- meta

  static final class Case {
    String id;
    String pkg;
    int k;
    String[] d1;
    String[] d2;
    String xs;
  }

  static AnnotationExprent metadataOf(StructClass cl) {
    for (Key<?> key : new Key<?>[] {StructGeneralAttribute.ATTRIBUTE_RUNTIME_VISIBLE_ANNOTATIONS, StructGeneralAttribute.ATTRIBUTE_RUNTIME_INVISIBLE_ANNOTATIONS}) {
      if (cl.hasAttribute(key)) {
        @SuppressWarnings("unchecked")
        StructAnnotationAttribute attr = cl.getAttribute((Key<StructAnnotationAttribute>) key);
        for (AnnotationExprent a : attr.getAnnotations()) {
          if ("kotlin/Metadata".equals(a.getClassName())) return a;
        }
      }
    }
    return null;
  }

  static String[] strings(Exprent e) {
    return ((NewExprent) e).getLstArrayElements().stream().map(x -> (String) ((ConstExprent) x).getValue()).toArray(String[]::new);
  }

  static Case extract(StructClass cl) {
    AnnotationExprent a = metadataOf(cl);
    if (a == null) return null;
    try {
      List<String> names = a.getParNames();
      List<? extends Exprent> values = a.getParValues();
      int ki = names.indexOf("k"), d1i = names.indexOf("d1"), d2i = names.indexOf("d2"), xsi = names.indexOf("xs");
      if (ki == -1 || d1i == -1) return null;
      Case c = new Case();
      c.k = (Integer) ((ConstExprent) values.get(ki)).getValue();
      c.d1 = strings(values.get(d1i));
      c.d2 = d2i == -1 ? null : strings(values.get(d2i));
      c.xs = xsi == -1 ? null : (String) ((ConstExprent) values.get(xsi)).getValue();
      int slash = cl.qualifiedName.lastIndexOf('/');
      c.pkg = slash < 0 ? "" : cl.qualifiedName.substring(0, slash);
      return c;
    } catch (RuntimeException e) {
      return null;
    }
  }

  static void meta(String label, String path, String listFile) throws Exception {
    Set<String> only = null;
    if (listFile != null) {
      only = new HashSet<>(Files.readAllLines(Paths.get(listFile)));
    }
    Log log = new Log();
    StructContext sc = install(log);
    sc.addSpace(new File(path), true);
    List<Case> cases = new ArrayList<>();
    for (String name : classNames(path)) {
      String id = label + "!" + name;
      if (only != null && !only.contains(id)) continue;
      StructClass cl;
      try {
        cl = sc.getClass(name);
      } catch (Throwable t) {
        continue;
      }
      if (cl == null) continue;
      Case c = extract(cl);
      if (c != null) {
        c.id = id;
        cases.add(c);
      }
    }
    for (Case c : cases) {
      p("=== " + c.id);
      p("pkg " + esc(c.pkg));
      p("k " + c.k);
      p("d1 " + c.d1.length);
      for (String s : c.d1) p(bytes(s));
      if (c.d2 == null) {
        p("d2 -");
      } else {
        p("d2 " + c.d2.length);
        for (String s : c.d2) p("s " + esc(s));
      }
      if (c.xs != null) p("xs " + esc(c.xs));
      p("--");
      walk(c);
      p("end");
    }
  }

  static String unesc(String s) {
    StringBuilder b = new StringBuilder();
    for (int i = 0; i < s.length(); i++) {
      char c = s.charAt(i);
      if (c == '\\') {
        b.append((char) Integer.parseInt(s.substring(i + 2, i + 6), 16));
        i += 5;
      } else {
        b.append(c);
      }
    }
    return b.toString();
  }

  static String d1String(String line) {
    if (line.startsWith("h ")) {
      StringBuilder b = new StringBuilder();
      for (int i = 2; i + 1 < line.length(); i += 2) b.append((char) Integer.parseInt(line.substring(i, i + 2), 16));
      return b.toString();
    }
    return unesc(line.substring(2));
  }

  static void metaRaw(String file) throws Exception {
    Iterator<String> it = Files.readAllLines(Paths.get(file)).iterator();
    while (it.hasNext()) {
      String header = it.next();
      if (header.isEmpty()) continue;
      Case c = new Case();
      c.id = header.substring(4);
      p(header);
      String line = it.next();
      p(line);
      c.pkg = unesc(line.substring(4));
      line = it.next();
      p(line);
      c.k = Integer.parseInt(line.substring(2));
      line = it.next();
      p(line);
      c.d1 = new String[Integer.parseInt(line.substring(3))];
      for (int i = 0; i < c.d1.length; i++) {
        line = it.next();
        p(line);
        c.d1[i] = d1String(line);
      }
      line = it.next();
      p(line);
      if (!line.equals("d2 -")) {
        c.d2 = new String[Integer.parseInt(line.substring(3))];
        for (int i = 0; i < c.d2.length; i++) {
          line = it.next();
          p(line);
          c.d2[i] = unesc(line.substring(2));
        }
      }
      line = it.next();
      if (line.startsWith("xs ")) {
        p(line);
        c.xs = unesc(line.substring(3));
        line = it.next();
      }
      p(line);
      walk(c);
      p("end");
    }
  }

  static void walk(Case c) {
    install(new Log());
    KotlinImportCollector kc = freshCollector(c.pkg);
    MetadataNameResolver r;
    Object msg;
    try {
      ByteArrayInputStream in = new ByteArrayInputStream(BitEncoding.decodeBytes(c.d1));
      r = c.d2 == null ? null : new MetadataNameResolver(JvmProtoBuf.StringTableTypes.parseDelimitedFrom(in, EXT), c.d2);
      msg = switch (c.k) {
        case 1 -> ProtoBuf.Class.parseFrom(in, EXT);
        case 2, 5 -> ProtoBuf.Package.parseFrom(in, EXT);
        case 3 -> ProtoBuf.Function.parseFrom(in, EXT);
        default -> null;
      };
    } catch (Throwable t) {
      p("decode " + exc(t));
      return;
    }
    if (r == null) {
      p("noresolver");
      return;
    }
    List<Runnable> tail = new ArrayList<>();
    if (msg instanceof ProtoBuf.Class pc) {
      walkClass(pc, r, tail);
    } else if (msg instanceof ProtoBuf.Package pp) {
      walkPackage(pp, r, tail);
    } else if (msg instanceof ProtoBuf.Function f) {
      p("lambda");
      walkFunction(f, f.getTypeTable(), r, tail);
    } else {
      p("nomsg");
    }
    TextBuffer ib = new TextBuffer();
    kc.writeImports(ib, true);
    p("imports " + esc(ib.convertToStringAndAllowDataDiscard()));
    for (Runnable t : tail) item(t);
  }

  static void walkClass(ProtoBuf.Class pc, MetadataNameResolver r, List<Runnable> tail) {
    p("class " + pc.getFlags());
    item(() -> p("fq " + esc(r.resolve(pc.getFqName()))));
    if (pc.hasCompanionObjectName()) item(() -> p("companion " + esc(r.resolve(pc.getCompanionObjectName()))));
    for (ProtoBuf.Type t : pc.getSupertypeList()) {
      item(() -> {
        p("super");
        top(KType.from(t, r), " ");
      });
    }
    for (ProtoBuf.Function f : pc.getFunctionList()) walkFunction(f, pc.getTypeTable(), r, tail);
    for (ProtoBuf.Property pr : pc.getPropertyList()) walkProperty(pr, r);
    for (ProtoBuf.Constructor ct : pc.getConstructorList()) walkConstructor(ct, r, tail);
    walkAliases(pc.getTypeAliasList(), r);
    walkTypeTable(pc.getTypeTable(), r);
  }

  static void walkPackage(ProtoBuf.Package pp, MetadataNameResolver r, List<Runnable> tail) {
    p("package");
    for (ProtoBuf.Function f : pp.getFunctionList()) walkFunction(f, pp.getTypeTable(), r, tail);
    for (ProtoBuf.Property pr : pp.getPropertyList()) walkProperty(pr, r);
    walkAliases(pp.getTypeAliasList(), r);
    walkTypeTable(pp.getTypeTable(), r);
  }

  static void walkAliases(List<ProtoBuf.TypeAlias> aliases, MetadataNameResolver r) {
    for (ProtoBuf.TypeAlias ta : aliases) {
      item(() -> {
        p("alias " + esc(r.resolve(ta.getName())));
        top(KType.from(ta.getUnderlyingType(), r), " ");
      });
    }
  }

  static void walkTypeTable(ProtoBuf.TypeTable tt, MetadataNameResolver r) {
    for (int i = 0; i < tt.getTypeCount(); i++) {
      int idx = i;
      item(() -> {
        p("tt " + idx);
        top(KType.from(idx, r, tt), " ");
      });
    }
  }

  static void dumpParams(KParameter[] params) {
    for (KParameter kp : params) {
      p(" param " + kp.flags() + " " + esc(kp.name()) + " " + kp.typeId());
      top(kp.type(), "  ");
      if (kp.varargType() != null) {
        p("  vararg");
        top(kp.varargType(), "   ");
      }
    }
  }

  static void paramStrings(String name, KParameter[] params, List<Runnable> tail) {
    tail.add(() -> {
      p("pstr " + esc(name));
      for (KParameter kp : params) {
        item(() -> {
          TextBuffer b = new TextBuffer();
          kp.stringify(1, b);
          p(" " + esc(b.convertToStringAndAllowDataDiscard()));
        });
      }
    });
  }

  static void walkFunction(ProtoBuf.Function f, ProtoBuf.TypeTable tt, MetadataNameResolver r, List<Runnable> tail) {
    item(() -> {
      String name = r.resolve(f.getName());
      p("fun " + esc(name) + " " + f.getFlags());
      KParameter[] params = new KParameter[f.getValueParameterCount()];
      for (int i = 0; i < params.length; i++) {
        ProtoBuf.ValueParameter vp = f.getValueParameter(i);
        int flags = vp.getFlags();
        String pname = r.resolve(vp.getName());
        KType type = KType.from(vp.getType(), r);
        KType vararg = vp.hasVarargElementType() ? KType.from(vp.getVarargElementType(), r) : null;
        params[i] = new KParameter(flags, pname, type, vararg, vp.getTypeId());
      }
      dumpParams(params);
      for (ProtoBuf.TypeParameter tp : f.getTypeParameterList()) {
        typeParameter(KTypeParameter.from(tp, r), " ");
      }
      if (f.hasReceiverType()) {
        p(" receiver");
        top(KType.from(f.getReceiverType(), r), "  ");
      }
      p(" returns");
      top(KType.from(f.getReturnType(), r), "  ");
      for (ProtoBuf.Type ctx : f.getContextReceiverTypeList()) {
        p(" context");
        top(KType.from(ctx, r), "  ");
      }
      paramStrings(name, params, tail);
      if (f.hasContract()) {
        tail.add(() -> {
          p("contract " + esc(name));
          KContract c = KContract.from(f.getContract(), List.of(params), r, tt);
          p(" " + esc(c.stringify(1).convertToStringAndAllowDataDiscard()));
        });
      }
    });
  }

  static void walkProperty(ProtoBuf.Property pr, MetadataNameResolver r) {
    item(() -> {
      p("prop " + esc(r.resolve(pr.getName())) + " " + pr.getFlags());
      KType t = KType.from(pr.getReturnType(), r);
      top(t, " ");
      p(" jsig " + esc(KTypes.getJavaSignature(t.kotlinType, pr.getReturnType().getNullable())));
      if (pr.hasSetterValueParameter()) {
        p(" setter");
        top(KType.from(pr.getSetterValueParameter().getType(), r), "  ");
      }
    });
  }

  static void walkConstructor(ProtoBuf.Constructor ct, MetadataNameResolver r, List<Runnable> tail) {
    item(() -> {
      p("ctor " + ct.getFlags());
      KParameter[] params = new KParameter[ct.getValueParameterCount()];
      for (int i = 0; i < params.length; i++) {
        ProtoBuf.ValueParameter vp = ct.getValueParameter(i);
        params[i] = new KParameter(
          vp.getFlags(),
          r.resolve(vp.getName()),
          KType.from(vp.getType(), r),
          KType.from(vp.getVarargElementType(), r),
          vp.getTypeId()
        );
      }
      dumpParams(params);
      paramStrings("<init>", params, tail);
    });
  }

  static void typeParameter(KTypeParameter tp, String ind) {
    p(ind + "tparam " + tp.reified() + " " + tp.variance().name() + " " + esc(tp.name()) + " " + tp.id());
    for (KType b : tp.upperBounds()) top(b, ind + " ");
  }

  static String ident(KType t) {
    if (t == KType.UNIT) return " =UNIT";
    if (t == KType.NOTHING) return " =NOTHING";
    if (t == KType.UBYTE) return " =UBYTE";
    if (t == KType.USHORT) return " =USHORT";
    if (t == KType.UINT) return " =UINT";
    if (t == KType.ULONG) return " =ULONG";
    return "";
  }

  static void type(KType t, String ind) {
    p(ind + "type " + esc(t.kotlinType) + " " + t.isNullable + " " + t.isUnsignedType + " " + esc(t.typeParameterName)
      + " " + esc(t.typeAliasName) + " " + esc(t.toString()) + " " + t.type.name() + " " + t.arrayDim + " "
      + t.typeFamily.name() + " " + t.stackSize + ident(t));
    if (t.typeArguments != null) {
      for (KType.TypeArgument a : t.typeArguments) {
        p(ind + " arg " + a.typeProjection().name());
        type(a.type(), ind + "  ");
      }
    }
  }

  /** A type as the plugin builds it, plus what the plugin prints of it. */
  static void top(KType t, String ind) {
    type(t, ind);
    p(ind + "jsig " + esc(KTypes.getJavaSignature(t.kotlinType, t.isNullable)));
    item(() -> p(ind + "kt " + esc(KTypes.getKotlinType(t))));
    String[] str = {null};
    item(() -> {
      str[0] = t.stringify(1).convertToStringAndAllowDataDiscard();
      p(ind + "str " + esc(str[0]));
    });
    item(() -> {
      TextBuffer b = new TextBuffer();
      b.append("#".repeat(150));
      b.append(t.stringify(1));
      b.reformat();
      String fmt = b.convertToStringAndAllowDataDiscard().substring(150);
      if (!fmt.equals(str[0])) p(ind + "fmt " + esc(fmt));
    });
  }

  // ---- chooser

  static String describe(KElement e) {
    if (e == null) return "-";
    if (e instanceof KClass c) return "class " + c.flags() + " " + (c.resolver() != null);
    if (e instanceof KFile f) return "file " + (f.resolver() != null) + " " + esc(f.multifileName());
    if (e instanceof KMultifileFacade m) return "facade " + m.files().stream().map(KtElementDump::esc).collect(Collectors.joining(" "));
    if (e instanceof KFunctionReference) return "funref";
    if (e == KFunction.FAILED_LAMBDA) return "failedlambda";
    if (e instanceof KFunction) return "function";
    if (e instanceof KHiddenElement h) return "hidden " + h.name();
    return e.getClass().getSimpleName();
  }

  static void chooser(String label, String dir) throws Exception {
    String[][] scenarios = {{"1", "0"}, {"0", "0"}, {"0", "1"}};
    for (String[] sc : scenarios) {
      String scenario = "e" + sc[0] + "x" + sc[1];
      Log log = new Log();
      StructContext ctx = install(log, "kt-enable", sc[0], "kt-export-metadata", sc[1]);
      ctx.addSpace(new File(dir), true);
      for (String name : classNames(dir)) {
        StructClass cl = ctx.getClass(name);
        p("=== " + label + "!" + scenario + "!" + name);
        for (int round = 0; round < 2; round++) {
          log.lines.clear();
          String res;
          try {
            res = String.valueOf(new KotlinChooser().isLanguage(cl));
          } catch (Throwable t) {
            res = exc(t);
          }
          p("call " + res);
          for (String l : log.lines) p("log " + l);
          p("elem " + describe(cl.getAttribute(KElement.KEY)));
        }
        p("end");
      }
    }
  }

  // ---- KTypes.getKotlinType

  static void addTypes(String signature, Set<String> into) {
    String s = signature;
    try {
      if (s.startsWith("<")) {
        int depth = 0;
        int i = 0;
        for (; i < s.length(); i++) {
          char c = s.charAt(i);
          if (c == '<') depth++;
          if (c == '>' && --depth == 0) break;
        }
        s = s.substring(i + 1);
      }
      if (s.startsWith("(")) {
        String params = s.substring(1, s.indexOf(')'));
        while (!params.isEmpty()) {
          String t = GenericType.getNextType(params);
          into.add(t);
          params = params.substring(t.length());
        }
        s = s.substring(s.indexOf(')') + 1);
        into.add(GenericType.getNextType(s));
      } else {
        into.add(s);
      }
    } catch (RuntimeException e) {
      // not a signature this split understands
    }
  }

  static final String[] CRAFTED_TYPES = {
    "Lkotlin/jvm/functions/Function0<Lkotlin/Unit;>;",
    "Lkotlin/jvm/functions/Function1<-Ljava/lang/String;+Ljava/lang/Integer;>;",
    "Lkotlin/jvm/functions/Function2<-Ljava/lang/Integer;-Lkotlin/jvm/functions/Function1<-TT;Lkotlin/Unit;>;Lkotlin/jvm/functions/Function0<TR;>;>;",
    "Lkotlin/jvm/functions/FunctionN<*>;",
    "Lkotlin/jvm/functions/FunctionN;",
    "Lkotlin/jvm/functions/Function3;",
    "Lkotlin/jvm/functions/Function2<Ljava/lang/String;>;",
    "Lkotlin/jvm/functions/Function1<*Ljava/lang/String;>;",
    "Lkotlin/jvm/functions/FunctionX<Ljava/lang/String;>;",
    "[Lkotlin/jvm/functions/Function1<Ljava/lang/Object;Ljava/lang/Object;>;",
    "Ljava/util/Map<Ljava/lang/String;Ljava/util/List<Ljava/lang/Integer;>;>;",
    "Ljava/util/Map;",
    "Ljava/util/HashMap<TK;TV;>;",
    "Ljava/util/Map$Entry<TK;TV;>;",
    "[[Ljava/util/List<+Ljava/lang/Number;>;",
    "Ljava/util/Set<-Ljava/lang/CharSequence;>;",
    "Ljava/lang/Iterable<*>;",
    "Ljava/util/Iterator<[I>;",
    "Ljava/util/Collection<[[Ljava/lang/String;>;",
    "Lfoo/Outer<TT;>.Inner<TU;>;",
    "Lfoo/Outer$Inner;",
    "Ljava/lang/Integer;",
    "[Ljava/lang/Object;",
    "Ljava/lang/Object;",
    "TT;",
    "[TT;",
    "I", "J", "Z", "B", "C", "S", "F", "D", "V",
    "[I", "[[Z", "[[[D",
    "Ljava/lang/String;",
    "Lkotlin/Unit;",
    "Ljava/lang/Void;",
  };

  static void kotlinTypes(String label, String path, int every) throws Exception {
    Set<String> types = new TreeSet<>();
    if (path.equals("-")) {
      types.addAll(Arrays.asList(CRAFTED_TYPES));
    } else {
      install(new Log()).addSpace(new File(path), true);
      StructContext sc = DecompilerContext.getStructContext();
      for (String name : classNames(path)) {
        StructClass cl;
        try {
          cl = sc.getClass(name);
        } catch (Throwable t) {
          continue;
        }
        if (cl == null) continue;
        for (StructMethod mt : cl.getMethods()) {
          addTypes(mt.getDescriptor(), types);
          StructGenericSignatureAttribute sig = mt.getAttribute(StructGeneralAttribute.ATTRIBUTE_SIGNATURE);
          if (sig != null) addTypes(sig.getSignature(), types);
        }
        for (StructField fd : cl.getFields()) {
          addTypes(fd.getDescriptor(), types);
          StructGenericSignatureAttribute sig = fd.getAttribute(StructGeneralAttribute.ATTRIBUTE_SIGNATURE);
          if (sig != null) addTypes(sig.getSignature(), types);
        }
      }
    }
    install(new Log());
    int n = 0;
    for (String s : types) {
      if (n++ % every != 0) continue;
      p("=== " + label + " " + esc(s));
      freshCollector("");
      VarType t;
      try {
        t = GenericType.parse(s);
      } catch (Throwable e) {
        p("parse " + exc(e));
        p("end");
        continue;
      }
      item(() -> p("kt " + esc(KTypes.getKotlinType(t))));
      item(() -> p("ko " + esc(KTypes.getKotlinType(t, false))));
      p("end");
    }
  }

  // ---- crafted inputs

  static final String[] SIGNATURE_INPUTS = {
    "kotlin/Int", "kotlin/Long", "kotlin/Short", "kotlin/Byte", "kotlin/Boolean", "kotlin/Char", "kotlin/Float",
    "kotlin/Double", "kotlin/Unit", "kotlin/Any", "kotlin/String", "kotlin/Nothing", "kotlin/Number",
    "kotlin/Int$Companion", "kotlin/Int.Companion", "kotlin/String.Companion", "kotlin/Unit$Companion",
    "kotlin/IntArray", "kotlin/BooleanArray", "kotlin/UnitArray", "kotlin/Array", "kotlin/ArrayArray",
    "kotlin/IntArrayArray", "kotlin/In$Companiont$Companion", "kotlin/$CompanionInt$Companion",
    "kotlin/collections/List", "kotlin/collections/MutableMap.MutableEntry", "kotlin/collections/Map.Entry",
    "kotlin/collections/Map$Entry", "kotlin/collections/Iterable", "kotlin/collections/ArrayList",
    "kotlin/Function", "kotlin/Function0", "kotlin/Function1", "kotlin/Function22", "kotlin/Function23",
    "kotlin/Function-1", "kotlin/Function+5", "kotlin/FunctionX", "kotlin/Function1a", "kotlin/Function99999999999",
    "kotlin/Function١", "kotlin/reflect/KFunction1", "kotlin/jvm/functions/Function1",
    "Lkotlin/Int;", "Lkotlin/collections/List;", "L;", "L", ";", "", "kotlin/", "kotlin",
    "foo/Bar", "foo/Bar.Baz", "foo/Bar.Baz.Qux", "java/lang/String", "Lfoo/Bar;", "kotlin.Int",
  };

  static void misc() {
    install(new Log());
    for (String s : SIGNATURE_INPUTS) {
      for (boolean nullable : new boolean[] {false, true}) {
        String res;
        try {
          res = esc(KTypes.getJavaSignature(s, nullable));
        } catch (Throwable t) {
          res = exc(t);
        }
        p("jsig " + esc(s) + " " + nullable + " " + res);
      }
    }
    for (String show : new String[] {"1", "0"}) {
      install(new Log(), "kt-show-public", show);
      for (ProtoBuf.Visibility v : ProtoBuf.Visibility.values()) {
        TextBuffer b = new TextBuffer();
        KUtils.appendVisibility(b, v);
        p("vis " + show + " " + v.name() + " " + esc(b.convertToStringAndAllowDataDiscard()));
      }
      String res;
      try {
        KUtils.appendVisibility(new TextBuffer(), null);
        res = "ok";
      } catch (Throwable t) {
        res = exc(t);
      }
      p("vis " + show + " null " + res);
    }
  }
}
