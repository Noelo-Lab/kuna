package corpus;

import net.fabricmc.fernflower.api.FabricJavadocStyle;
import net.fabricmc.fernflower.api.IFabricJavadocProvider;
import org.jetbrains.java.decompiler.main.decompiler.ConsoleDecompiler;
import org.jetbrains.java.decompiler.main.decompiler.PrintStreamLogger;
import org.jetbrains.java.decompiler.main.extern.IFernflowerLogger;
import org.jetbrains.java.decompiler.main.extern.IFernflowerPreferences;
import org.jetbrains.java.decompiler.main.plugins.PluginSources;
import org.jetbrains.java.decompiler.struct.StructClass;
import org.jetbrains.java.decompiler.struct.StructField;
import org.jetbrains.java.decompiler.struct.StructMethod;
import org.jetbrains.java.decompiler.util.TextBuffer;

import java.io.File;
import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.TreeMap;
import java.util.stream.Stream;

/**
 * Drives Vineflower 1.12.0 through the same API path as testFixtures/.../DecompilerTestFixture:
 * a ConsoleDecompiler subclass built with (destination, options, PrintStreamLogger(System.out), saveType),
 * addSource(File) per input in the given order, decompileContext(), TextBuffer.checkLeaks(), close().
 *
 * Usage: VfDriver [--fixture] [--opt k=v]... [--javadoc plain|markdown] [--plugins]
 *                 [--save legacy|folder|file|console] [--lib PATH]... [--inputs-from FILE]
 *                 [--print-options] INPUT... -o OUT
 */
public final class VfDriver {
  static final class Decompiler extends ConsoleDecompiler {
    Decompiler(File destination, Map<String, Object> options, IFernflowerLogger logger, SaveType saveType) {
      super(destination, options, logger, saveType);
    }
  }

  /** The option map DecompilerTestFixture.setUp() starts from before applying a test set's pairs. */
  static Map<String, Object> fixtureBase() {
    Map<String, Object> options = new LinkedHashMap<>();
    options.put(IFernflowerPreferences.LOG_LEVEL, "warn");
    options.put(IFernflowerPreferences.DECOMPILE_GENERIC_SIGNATURES, "1");
    options.put(IFernflowerPreferences.REMOVE_SYNTHETIC, "1");
    options.put(IFernflowerPreferences.REMOVE_BRIDGE, "1");
    options.put(IFernflowerPreferences.LITERALS_AS_IS, "1");
    options.put(IFernflowerPreferences.UNIT_TEST_MODE, "1");
    options.put(IFernflowerPreferences.NEW_LINE_SEPARATOR, "1");
    options.put(IFernflowerPreferences.ERROR_MESSAGE, "");
    options.put(IFernflowerPreferences.INCLUDE_JAVA_RUNTIME, "");
    return options;
  }

  /** The anonymous IFabricJavadocProvider instances registered by SingleClassesTest's "Javadoc" sets. */
  static IFabricJavadocProvider javadocProvider(boolean markdown) {
    return new IFabricJavadocProvider() {
      @Override
      public String getClassDoc(StructClass structClass) {
        return "Class javadoc for '" + structClass.qualifiedName + "'";
      }

      @Override
      public String getFieldDoc(StructClass structClass, StructField structField) {
        return "Field javadoc for '" + structField.getName() + "'";
      }

      @Override
      public String getMethodDoc(StructClass structClass, StructMethod structMethod) {
        return "Method javadoc for '" + structMethod.getName() + "'";
      }

      @Override
      public FabricJavadocStyle getClassJavadocStyle(StructClass structClass) {
        return markdown ? FabricJavadocStyle.MARKDOWN : FabricJavadocStyle.HTML;
      }
    };
  }

  private static void usage(String msg) {
    System.err.println("error: " + msg);
    System.err.println("usage: vf-decompile [--fixture] [--opt k=v]... [--javadoc plain|markdown] [--plugins]");
    System.err.println("                    [--save legacy|folder|file|console] [--lib PATH]... [--inputs-from FILE]");
    System.err.println("                    [--print-options] INPUT... -o OUT");
    System.exit(64);
  }

  public static void main(String[] args) throws IOException {
    Map<String, Object> options = new LinkedHashMap<>();
    List<String> optPairs = new ArrayList<>();
    List<File> inputs = new ArrayList<>();
    List<File> libs = new ArrayList<>();
    String javadoc = null;
    boolean fixture = false;
    boolean plugins = false;
    boolean printOptions = false;
    String save = "legacy";
    String out = null;

    for (int i = 0; i < args.length; i++) {
      String a = args[i];
      switch (a) {
        case "--fixture": fixture = true; break;
        case "--plugins": plugins = true; break;
        case "--print-options": printOptions = true; break;
        case "--opt":
          if (++i >= args.length) usage("--opt needs k=v");
          optPairs.add(args[i]);
          break;
        case "--javadoc":
          if (++i >= args.length) usage("--javadoc needs plain|markdown");
          javadoc = args[i];
          if (!javadoc.equals("plain") && !javadoc.equals("markdown")) usage("bad --javadoc " + javadoc);
          break;
        case "--save":
          if (++i >= args.length) usage("--save needs a mode");
          save = args[i];
          break;
        case "--lib":
          if (++i >= args.length) usage("--lib needs a path");
          libs.add(new File(args[i]));
          break;
        case "--inputs-from":
          if (++i >= args.length) usage("--inputs-from needs a file");
          for (String line : Files.readAllLines(Paths.get(args[i]), StandardCharsets.UTF_8)) {
            if (!line.isEmpty()) inputs.add(new File(line));
          }
          break;
        case "-o":
          if (++i >= args.length) usage("-o needs a directory");
          out = args[i];
          break;
        default:
          if (a.startsWith("--opt=")) {
            optPairs.add(a.substring(6));
          } else if (a.startsWith("--") ) {
            usage("unknown flag " + a);
          } else {
            inputs.add(new File(a));
          }
      }
    }
    if (out == null) usage("missing -o OUT");
    if (inputs.isEmpty()) usage("no inputs");
    for (File f : inputs) {
      if (!f.exists()) usage("input does not exist: " + f);
    }

    if (fixture) options.putAll(fixtureBase());
    for (String kv : optPairs) {
      int eq = kv.indexOf('=');
      if (eq <= 0) usage("--opt expects k=v, got " + kv);
      options.put(kv.substring(0, eq), kv.substring(eq + 1));
    }
    if (javadoc != null) {
      options.put(IFabricJavadocProvider.PROPERTY_NAME, javadocProvider(javadoc.equals("markdown")));
    }

    ConsoleDecompiler.SaveType saveType;
    switch (save) {
      case "legacy": saveType = ConsoleDecompiler.SaveType.LEGACY_CONSOLEDECOMPILER; break;
      case "folder": saveType = ConsoleDecompiler.SaveType.FOLDER; break;
      case "file": saveType = ConsoleDecompiler.SaveType.FILE; break;
      case "console": saveType = ConsoleDecompiler.SaveType.CONSOLE; break;
      default: usage("bad --save " + save); return;
    }

    File dest = new File(out);
    if (saveType == ConsoleDecompiler.SaveType.FILE) {
      if (dest.getParentFile() != null) Files.createDirectories(dest.getParentFile().toPath());
    } else if (saveType != ConsoleDecompiler.SaveType.CONSOLE) {
      Path d = dest.toPath();
      if (Files.isDirectory(d)) {
        try (Stream<Path> s = Files.list(d)) {
          if (s.findAny().isPresent()) usage("output directory is not empty: " + d);
        }
      }
      Files.createDirectories(d);
    }

    if (!plugins) {
      // The Gradle test classpath runs Vineflower from class directories with no plugin jars/services,
      // so no plugin is ever registered there; the release jar would load META-INF/plugins/*.jar.
      PluginSources.PLUGIN_SOURCES.clear();
    }

    if (printOptions) {
      Map<String, Object> sorted = new TreeMap<>(options);
      for (Map.Entry<String, Object> e : sorted.entrySet()) {
        Object v = e.getValue();
        System.err.println("option " + e.getKey() + "=" + (v instanceof String ? "\"" + v + "\"" : v.getClass().getName()));
      }
      System.err.println("plugins " + (plugins ? "enabled" : "disabled") + ", save " + save);
    }

    Decompiler decompiler = new Decompiler(dest, options, new PrintStreamLogger(System.out), saveType);
    try {
      for (File lib : libs) decompiler.addLibrary(lib);
      for (File in : inputs) decompiler.addSource(in);
      decompiler.decompileContext();
      TextBuffer.checkLeaks();
    } finally {
      try {
        decompiler.close();
      } catch (Exception e) {
        throw new RuntimeException(e);
      }
    }
    System.out.flush();
  }
}
