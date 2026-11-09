// Golden vectors for the context-dependent TextBuffer / TextUtil / GenericType text paths, from
// Vineflower 1.12.0 itself, inside a context made the way Fernflower's constructor makes one (no runtime):
//
//   javac -cp vineflower-1.12.0.jar -d GEN_DIR GenTextContextGolden.java
//   java -cp vineflower-1.12.0.jar:GEN_DIR GenTextContextGolden f6/tree/pkg/Hello.class > text_context_golden.txt
//
// The current class node is pkg.Hello (the argument), and every row runs on a fresh ImportCollector.
// Rows are tab-separated, strings escaped (backslash, tab, CR, LF and every unit outside 0x20..0x7e as in esc(); null = \N);
// a throw is "EXC:<simple class name>:<message>". A dump is the text plus TextTokenDumpVisitor's listing.
//   IS indent length result                         TextUtil.getIndentString
//   KW id major static isKeyword isValidIdentifier  TextUtil on a pkg.Hello method (main / <init>)
//   QS qualifier dump                               TextUtil.writeQualifiedSuper
//   CN sig castName                                 GenericType.getCastName (GenericType instances only)
//   TS sig toString                                 VarType/GenericType.toString
//   CT sig castTypeName dump                        appendCastTypeName(ExprProcessor.getCastTypeName(t), t)
//   TN sig typeName dump                            appendTypeName(ExprProcessor.getTypeName(t), t)
//   CT0 sig dump / TN0 sig dump                     appendCastTypeName(t) / appendTypeName(t)
//   TT sig dump                                     "= " + getTypeName(t), then addTypeNameToken(t, 2)
//   TK kind value decl class name desc param index var dump   appendField / appendMethod / appendVariable
import org.jetbrains.java.decompiler.code.BytecodeVersion;
import org.jetbrains.java.decompiler.main.ClassesProcessor;
import org.jetbrains.java.decompiler.main.DecompilerContext;
import org.jetbrains.java.decompiler.main.Fernflower;
import org.jetbrains.java.decompiler.main.collectors.ImportCollector;
import org.jetbrains.java.decompiler.main.extern.IFernflowerLogger;
import org.jetbrains.java.decompiler.main.extern.IFernflowerPreferences;
import org.jetbrains.java.decompiler.main.extern.TextTokenVisitor;
import org.jetbrains.java.decompiler.modules.decompiler.ExprProcessor;
import org.jetbrains.java.decompiler.struct.StructClass;
import org.jetbrains.java.decompiler.struct.StructMethod;
import org.jetbrains.java.decompiler.struct.gen.VarType;
import org.jetbrains.java.decompiler.struct.gen.generics.GenericType;
import org.jetbrains.java.decompiler.util.DataInputFullStream;
import org.jetbrains.java.decompiler.util.TextBuffer;
import org.jetbrains.java.decompiler.util.TextUtil;
import org.jetbrains.java.decompiler.util.token.TextTokenDumpVisitor;

import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.HashMap;
import java.util.Map;
import java.util.function.Supplier;

public class GenTextContextGolden {
  static final StringBuilder OUT = new StringBuilder();
  static ClassesProcessor.ClassNode NODE;

  static String esc(String s) {
    if (s == null) return "\\N";
    StringBuilder b = new StringBuilder();
    for (int i = 0; i < s.length(); i++) {
      char c = s.charAt(i);
      if (c == '\\') b.append("\\\\");
      else if (c == '\t') b.append("\\t");
      else if (c == '\n') b.append("\\n");
      else if (c == '\r') b.append("\\r");
      else if (c < 0x20 || c > 0x7e) b.append(String.format("\\u%04x", (int) c));
      else b.append(c);
    }
    return b.toString();
  }

  static String run(Supplier<String> op) {
    try {
      return esc(op.get());
    } catch (Throwable t) {
      return "EXC:" + t.getClass().getSimpleName() + ":" + esc(t.getMessage());
    }
  }

  static void fresh() {
    DecompilerContext.startClass(new ImportCollector(NODE));
  }

  static String dump(TextBuffer buf) {
    TextBuffer[] holder = {buf};
    buf.visitTokens(TextTokenVisitor.createVisitor(next -> new TextTokenDumpVisitor(next, holder[0])));
    return buf.convertToStringAndAllowDataDiscard();
  }

  static void row(String... fields) {
    OUT.append(String.join("\t", fields)).append('\n');
  }

  static final String[] SIGS = {
    "I", "[[J", "Z", "Ljava/lang/String;", "[Ljava/lang/String;", "Ljava/util/Map$Entry;", "[[Ljava/util/Map$Entry;",
    "La/b/Outer$Inner$Deep;", "Lpkg/Hello;", "Lpkg/Hello$1;", "LTopLevel;", "TT;", "[TT;",
    "Ljava/util/List<TT;>;", "Ljava/util/Map<TK;TV;>;", "Ljava/util/List<*>;", "Ljava/util/List<+TT;>;",
    "Ljava/util/List<-TT;>;", "Ljava/util/List<+Ljava/lang/Number;>;", "Ljava/util/List<-Ljava/lang/Integer;>;",
    "Ljava/util/Map<TK;Ljava/util/List<TV;>;>;", "Ljava/util/List<Ljava/util/List<-Ljava/lang/Integer;>;>;",
    "Ljava/util/Map$Entry<TK;*>;", "Lpkg/Outer<TT;>.Inner<TU;>;", "Lpkg/Outer<TT;>.Inner;", "Lpkg/Outer.Inner<TU;>;",
    "[Ljava/util/List<TT;>;", "[[Ljava/util/Map$Entry<TK;+TV;>;", "Ljava/util/List<[TT;>;", "Ljava/util/List<[[TT;>;",
    "Ljava/util/Map<+Ljava/util/List<*>;-Ljava/awt/List;>;", "Ljava/util/List<Ljava/lang/String;>;",
    "Ljava/util/Map<Ljava/util/List;Ljava/awt/List;>;", "Ljava/util/List<[I>;", "Ljava/util/List<+[Ljava/lang/String;>;",
    "Lx/Café<TÉté;>;", "Ljava/util/function/Function<-TT;+TR;>;",
  };

  public static void main(String[] args) throws Exception {
    Map<String, Object> props = new HashMap<>();
    props.put(IFernflowerPreferences.INCLUDE_JAVA_RUNTIME, "");
    new Fernflower(null, props, IFernflowerLogger.NO_OP);

    byte[] bytes = Files.readAllBytes(Paths.get(args[0]));
    StructClass cl = StructClass.create(new DataInputFullStream(bytes), true);
    NODE = new ClassesProcessor.ClassNode(ClassesProcessor.ClassNode.Type.ROOT, cl);
    DecompilerContext.setProperty(DecompilerContext.CURRENT_CLASS_NODE, NODE);
    DecompilerContext.setProperty(DecompilerContext.CURRENT_CLASS, cl);
    StructMethod main = cl.getMethod("main", "([Ljava/lang/String;)V");
    StructMethod init = cl.getMethod("<init>", "()V");

    String[] indents = {"   ", "\t", "", "ab", "  é"};
    for (String indent : indents) {
      DecompilerContext.setProperty(IFernflowerPreferences.INDENT_STRING, indent);
      for (int n : new int[]{0, 1, 2, 5, -1}) {
        row("IS", esc(indent), Integer.toString(n), run(() -> TextUtil.getIndentString(n)));
      }
    }
    DecompilerContext.setProperty(IFernflowerPreferences.INDENT_STRING, "   ");

    String[] ids = {"abstract", "default", "if", "private", "this", "boolean", "do", "implements", "protected", "throw",
      "break", "double", "import", "public", "throws", "byte", "else", "instanceof", "return", "transient", "case",
      "extends", "int", "short", "try", "catch", "final", "interface", "static", "void", "char", "finally", "long",
      "strictfp", "volatile", "class", "float", "native", "super", "while", "const", "for", "new", "switch",
      "continue", "goto", "package", "synchronized", "true", "false", "null", "assert", "enum", "var", "record",
      "yield", "sealed", "permits", "module", "non-sealed", "_", "$", "$x", "x$", "Foo", "foo1", "1foo", "", " ",
      "a b", "a-b", "a.b", "café", "日本", "\u0000a", "a\u0000", "a​", "²x", "x²",
      "€sym", "𝐀", "This", "ENUM", "Enum"};
    int[] majors = {45, 48, 49, 52, 65};
    for (String id : ids) {
      for (int major : majors) {
        BytecodeVersion version = new BytecodeVersion(major, 0);
        for (StructMethod mt : new StructMethod[]{main, init}) {
          row("KW", esc(id), Integer.toString(major), mt == main ? "1" : "0",
            run(() -> String.valueOf(TextUtil.isKeyword(id, version, mt))),
            run(() -> String.valueOf(TextUtil.isValidIdentifier(id, version, mt))));
        }
      }
    }

    for (String q : new String[]{"pkg/Hello", "java/lang/Object", "pkg/Other", "a/b/Outer$Inner", "TopLevel", "pkg/Hello$1"}) {
      fresh();
      row("QS", esc(q), run(() -> {
        TextBuffer buf = new TextBuffer();
        TextUtil.writeQualifiedSuper(buf, q);
        return dump(buf);
      }));
    }

    for (String sig : SIGS) {
      VarType t = GenericType.parse(sig);
      if (t instanceof GenericType g) {
        fresh();
        row("CN", esc(sig), run(g::getCastName));
      }
      fresh();
      row("TS", esc(sig), run(t::toString));

      fresh();
      String castName = run(() -> ExprProcessor.getCastTypeName(t));
      fresh();
      row("CT", esc(sig), castName, run(() -> dump(new TextBuffer().appendCastTypeName(ExprProcessor.getCastTypeName(t), t))));
      fresh();
      String typeName = run(() -> ExprProcessor.getTypeName(t));
      fresh();
      row("TN", esc(sig), typeName, run(() -> dump(new TextBuffer().appendTypeName(ExprProcessor.getTypeName(t), t))));

      fresh();
      row("CT0", esc(sig), run(() -> dump(new TextBuffer().appendCastTypeName(t))));
      fresh();
      row("TN0", esc(sig), run(() -> dump(new TextBuffer().appendTypeName(t))));
      fresh();
      row("TT", esc(sig), run(() -> {
        TextBuffer buf = new TextBuffer().append("= ").append(ExprProcessor.getTypeName(t));
        return dump(buf.addTypeNameToken(t, 2));
      }));
    }

    String[][] tokens = {
      {"F", "count", "1", "pkg/Hello", "count", "I", "", "", ""},
      {"F", "names", "0", "pkg/Hello", "names", "[Ljava/lang/String;", "", "", ""},
      {"F", "Map.EMPTY", "0", "java/util/Map", "EMPTY", "Ljava/util/Map$Entry;", "", "", ""},
      {"F", "café", "1", "x/Café", "café", "[[D", "", "", ""},
      {"M", "main", "1", "pkg/Hello", "main", "([Ljava/lang/String;)V", "", "", ""},
      {"M", "get", "0", "java/util/Map", "get", "(Ljava/lang/Object;)Ljava/lang/Object;", "", "", ""},
      {"M", "<init>", "1", "pkg/Hello", "<init>", "()V", "", "", ""},
      {"M", "mix", "0", "a/B", "mix", "(IJ[ZLjava/util/List;D)[[Ljava/lang/String;", "", "", ""},
      {"V", "args", "1", "pkg/Hello", "main", "([Ljava/lang/String;)V", "1", "0", "args"},
      {"V", "var1", "0", "pkg/Hello", "main", "([Ljava/lang/String;)V", "0", "1", "var1"},
      {"V", "x", "1", "a/B", "mix", "(IJ[ZLjava/util/List;D)[[Ljava/lang/String;", "0", "7", "x"},
      {"V", "été", "0", "a/B", "f", "(I)I", "1", "1", "été"},
    };
    for (String[] tk : tokens) {
      boolean decl = tk[2].equals("1");
      row("TK", esc(tk[0]), esc(tk[1]), tk[2], esc(tk[3]), esc(tk[4]), esc(tk[5]), tk[6], tk[7], esc(tk[8]), run(() -> {
        TextBuffer buf = new TextBuffer();
        buf.append("int a;").appendLineSeparator().appendIndent(1).append("x = ");
        switch (tk[0]) {
          case "F" -> buf.appendField(tk[1], decl, tk[3], tk[4], tk[5]);
          case "M" -> buf.appendMethod(tk[1], decl, tk[3], tk[4], tk[5]);
          default -> buf.appendVariable(tk[1], decl, tk[6].equals("1"), tk[3], tk[4], tk[5], Integer.parseInt(tk[7]), tk[8]);
        }
        buf.append(";").appendLineSeparator();
        return dump(buf);
      }));
    }

    System.out.print(OUT);
  }
}
