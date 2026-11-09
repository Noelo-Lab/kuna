// Golden vectors for tests/b19_classes.rs: the class node trees ClassesProcessor.loadClasses
// builds (with its warnings) under several options, the lambda nodes LambdaProcessor adds,
// the parameter names ClassWrapper.init gives methods without code, and
// Pattern.matches for the regex syntax the class filters use. Run through gen.sh.
import java.io.File;
import java.util.ArrayList;
import java.util.Comparator;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.regex.Pattern;
import java.util.regex.PatternSyntaxException;

import org.jetbrains.java.decompiler.main.ClassesProcessor.ClassNode;
import org.jetbrains.java.decompiler.main.DecompilerContext;
import org.jetbrains.java.decompiler.main.Fernflower;
import org.jetbrains.java.decompiler.main.collectors.ImportCollector;
import org.jetbrains.java.decompiler.main.extern.IFernflowerLogger;
import org.jetbrains.java.decompiler.main.extern.IFernflowerPreferences;
import org.jetbrains.java.decompiler.main.rels.ClassWrapper;
import org.jetbrains.java.decompiler.main.rels.LambdaProcessor;
import org.jetbrains.java.decompiler.main.rels.MethodWrapper;
import org.jetbrains.java.decompiler.modules.decompiler.vars.VarProcessor;
import org.jetbrains.java.decompiler.modules.decompiler.vars.VarVersionPair;

public class GenB19Oracle {
  static final List<String> log = new ArrayList<>();

  static String esc(String s) {
    if (s == null) return "\\N";
    return s.replace("\\", "\\\\").replace("\t", "\\t").replace("\n", "\\n");
  }

  static String node(ClassNode n) {
    List<String> nested = new ArrayList<>();
    for (ClassNode c : n.nested) nested.add(id(c));
    List<String> enclosing = new ArrayList<>(n.enclosingClasses);
    enclosing.sort(null);
    return String.join("\t", id(n), n.type.name(), esc(n.simpleName), Integer.toString(n.access),
      n.parent == null ? "\\N" : id(n.parent), String.join(",", nested), esc(n.enclosingMethod),
      String.join(",", enclosing), n.anonymousClassType == null ? "\\N" : n.anonymousClassType.value);
  }

  static String id(ClassNode n) {
    return n.type == ClassNode.Type.LAMBDA ? n.simpleName : n.classStruct.qualifiedName;
  }

  static String lambda(ClassNode n) {
    ClassNode.LambdaInformation l = n.lambdaInformation;
    return String.join("\t", "lambda", n.simpleName, l.method_name, l.method_descriptor,
      l.content_class_name, l.content_method_name, l.content_method_descriptor,
      Integer.toString(l.content_method_invocation_type), l.content_method_key,
      Boolean.toString(l.is_method_reference), Boolean.toString(l.is_content_method_static));
  }

  static List<File> classFiles(File dir) {
    List<File> out = new ArrayList<>();
    collect(dir, out);
    out.sort(Comparator.comparing(File::getPath));
    return out;
  }

  static void collect(File f, List<File> out) {
    if (f.isDirectory()) {
      for (File c : f.listFiles()) collect(c, out);
    } else if (f.getName().endsWith(".class")) {
      out.add(f);
    }
  }

  static Fernflower open(List<File> files, String... kv) {
    Map<String, Object> options = new HashMap<>();
    options.put(IFernflowerPreferences.INCLUDE_JAVA_RUNTIME, "0");
    for (int i = 0; i < kv.length; i += 2) options.put(kv[i], kv[i + 1]);
    IFernflowerLogger logger = new IFernflowerLogger() {
      @Override public void writeMessage(String message, Severity severity) {
        if (severity.ordinal() >= Severity.WARN.ordinal()) log.add("log\t" + severity.name() + "\t" + esc(message));
      }
      @Override public void writeMessage(String message, Severity severity, Throwable t) {
        writeMessage(message, severity);
      }
    };
    Fernflower fernflower = new Fernflower(null, options, logger);
    for (File f : files) fernflower.addSource(f);
    log.clear();
    return fernflower;
  }

  static void load(String name, List<File> files, String... kv) {
    Fernflower fernflower = open(files, kv);
    DecompilerContext.getClassProcessor().loadClasses(null);
    System.out.println("== load " + name);
    Map<String, ClassNode> roots = DecompilerContext.getClassProcessor().getMapRootClasses();
    List<String> names = new ArrayList<>(roots.keySet());
    names.sort(null);
    for (String n : names) System.out.println(node(roots.get(n)));
    for (String l : log) System.out.println(l);
    DecompilerContext.setCurrentContext(null);
  }

  static void dump(ClassNode n) {
    System.out.println(node(n));
    if (n.type == ClassNode.Type.LAMBDA) System.out.println(lambda(n));
    for (ClassNode c : n.nested) {
      if (c.parent == n) dump(c);
    }
  }

  public static void main(String[] args) throws Exception {
    File dir = new File(args[0]);
    List<File> own = classFiles(new File(dir, "classes"));
    List<File> inconsistent = classFiles(new File(dir, "inconsistent"));
    List<File> reversed = new ArrayList<>(inconsistent);
    reversed.sort(Comparator.comparing(File::getPath).reversed());

    load("default", own);
    load("verify-anonymous", own, IFernflowerPreferences.VERIFY_ANONYMOUS_CLASSES, "1");
    load("no-inner", own, IFernflowerPreferences.DECOMPILE_INNER, "0");
    load("excluded", own, IFernflowerPreferences.EXCLUDED_CLASSES, "b/Outer\\$A.*");
    load("included", own, IFernflowerPreferences.INCLUDED_CLASSES, "b/(Natives|Outer(\\$[0-9]+)?)");
    load("no-name-check", own, IFernflowerPreferences.VALIDATE_INNER_CLASSES_NAMES, "0");
    load("inconsistent", inconsistent);
    load("inconsistent-reversed", reversed);
    load("inconsistent-quiet", reversed, IFernflowerPreferences.WARN_INCONSISTENT_INNER_CLASSES, "0");

    Fernflower fernflower = open(own);
    DecompilerContext.getClassProcessor().loadClasses(null);
    Map<String, ClassNode> roots = DecompilerContext.getClassProcessor().getMapRootClasses();
    List<String> names = new ArrayList<>(roots.keySet());
    names.sort(null);
    System.out.println("== lambdas");
    for (String n : names) {
      ClassNode root = roots.get(n);
      if (root.type == ClassNode.Type.ROOT) new LambdaProcessor().processClass(root);
    }
    for (String n : names) {
      ClassNode root = roots.get(n);
      if (root.type == ClassNode.Type.ROOT) dump(root);
    }
    List<String> lambdaKeys = new ArrayList<>();
    for (String k : roots.keySet()) if (k.contains("##")) lambdaKeys.add(k);
    lambdaKeys.sort(null);
    System.out.println("keys\t" + String.join(",", lambdaKeys));

    System.out.println("== wrapper");
    ClassNode natives = roots.get("b/Natives");
    DecompilerContext.startClass(new ImportCollector(natives));
    ClassWrapper wrapper = new ClassWrapper(natives);
    wrapper.init(null);
    for (MethodWrapper mw : wrapper.getMethods()) {
      if (mw.methodStruct.containsCode()) continue;
      VarProcessor vp = mw.varproc;
      System.out.println(String.join("\t", "method", mw.methodStruct.getName(), mw.methodStruct.getDescriptor(),
        Boolean.toString(mw.decompileError == null), Boolean.toString(mw.root == null)));
      List<VarVersionPair> pairs = new ArrayList<>(vp.getUsedVarVersions());
      pairs.sort(Comparator.comparingInt((VarVersionPair p) -> p.var).thenComparingInt(p -> p.version));
      for (VarVersionPair p : pairs) System.out.println("var\t" + p.var + "\t" + p.version + "\t" + vp.getVarName(p));
      for (VarVersionPair p : vp.getParams()) System.out.println("param\t" + p.var + "\t" + p.version);
      List<VarVersionPair> thisVars = new ArrayList<>(vp.getThisVars().keySet());
      for (VarVersionPair p : thisVars) System.out.println("this\t" + p.var + "\t" + p.version + "\t" + vp.getThisVars().get(p));
    }
    DecompilerContext.setCurrentContext(null);

    System.out.println("== regex");
    String[][] cases = {
      {"b/Outer\\$A.*", "b/Outer$A$1"}, {"b/Outer\\$A.*", "b/Outer"}, {".*Test", "a/FooTest"},
      {"(a|b)/.*", "b/x"}, {"[a-c]+/X", "abc/X"}, {"[^a-c]+/X", "abc/X"}, {"\\w+/\\w+", "pkg/Name1"},
      {"\\d{2,3}", "1234"}, {"\\d{2,3}", "123"}, {"\\d{2,}", "1234"}, {"a*+a", "aaa"}, {"a*?a", "aaa"},
      {"a++", "aaa"}, {"(?:ab)*", "ababab"}, {"(?=b).*", "bx"}, {"(?!b).*", "bx"}, {"\\Qa.b\\E", "a.b"},
      {"\\Qa.b\\E", "axb"}, {"\\Qab\\E*", "abbb"}, {"x\\.y", "x.y"}, {"x.y", "x\ny"}, {"^a$", "a"},
      {"a$", "a\n"}, {"a{3}", "aaa"}, {"[\\w$]+", "Outer$1"}, {"[a-z[0-9]]+", "a1b2"},
      {"\\bfoo\\b.*", "foo bar"}, {"\\Bfoo", "foo"}, {"(?>a*)a", "aaa"}, {"\\u0041\\x42\\0103", "ABC"},
      {"(?<name>ab)+", "abab"}, {"[]a]+", "]a]"}, {"a|", ""}, {"(a|ab)(c|bcd)(d*)", "abcd"},
      {"[\\s\\S]*", "a b\nc"}, {"[a-]+", "a-a"}, {"\\D\\W\\S", "x y"}, {"\\t\\$", "\t$"}, {"(a*)*", "aaa"},
      {"(a?)+b", "b"}, {"\\Ab\\z", "b"}, {"a\\Z\\n", "a\n"}, {"a\\z\\n", "a\n"}, {"a$\\n", "a\n"}, {"a**", "a"}, {"(a", "a"}, {"[a", "a"}, {"*a", "a"},
      {"a{2", "aa"}, {"\\y", "y"}, {"a)", "a"}, {"[b-a]", "a"}, {"a{3,2}", "aaa"}};
    for (String[] c : cases) {
      String result;
      try {
        result = Boolean.toString(Pattern.compile(c[0]).matcher("").reset(c[1]).matches());
      } catch (PatternSyntaxException e) {
        result = "error";
      }
      System.out.println(String.join("\t", "regex", esc(c[0]), esc(c[1]), result));
    }
  }
}
