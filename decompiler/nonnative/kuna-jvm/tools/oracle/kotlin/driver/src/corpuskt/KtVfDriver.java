package corpuskt;

import org.jetbrains.java.decompiler.api.plugin.Plugin;
import org.jetbrains.java.decompiler.api.plugin.PluginSource;
import org.jetbrains.java.decompiler.main.decompiler.ConsoleDecompiler;
import org.jetbrains.java.decompiler.main.decompiler.PrintStreamLogger;
import org.jetbrains.java.decompiler.main.extern.IFernflowerLogger;
import org.jetbrains.java.decompiler.main.extern.IFernflowerPreferences;
import org.jetbrains.java.decompiler.main.plugins.PluginSources;
import org.jetbrains.java.decompiler.util.TextBuffer;

import java.io.File;
import java.io.IOException;
import java.net.URL;
import java.net.URLClassLoader;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.ServiceLoader;
import java.util.TreeMap;
import java.util.stream.Stream;

/**
 * Copy of the Java corpus driver, java/driver/src/corpus/VfDriver.java (DecompilerTestFixture's API path:
 * ConsoleDecompiler subclass with (destination, options, PrintStreamLogger(System.out), saveType),
 * addSource(File) per input in order, decompileContext(), TextBuffer.checkLeaks(), close()),
 * extended with --plugin-jar: load ONLY the plugins that a ServiceLoader finds in the given jars,
 * which is how the Kotlin plugin's own Gradle test run sees plugins (ServicePluginSource over the
 * plugin project's classes/resources; the root project's jar carries no META-INF/plugins).
 *
 * Usage: KtVfDriver [--fixture] [--opt k=v]... [--plugins | --plugin-jar JAR...]
 *                   [--save legacy|folder|file|console] [--lib PATH]... [--inputs-from FILE]
 *                   [--print-options] INPUT... -o OUT
 */
public final class KtVfDriver {
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

  /** Plugins found by ServiceLoader in exactly these jars (not in the parent class path). */
  static final class JarServicePluginSource implements PluginSource {
    private final URLClassLoader loader;

    JarServicePluginSource(List<File> jars) throws IOException {
      URL[] urls = new URL[jars.size()];
      for (int i = 0; i < urls.length; i++) urls[i] = jars.get(i).toURI().toURL();
      this.loader = new URLClassLoader(urls, KtVfDriver.class.getClassLoader());
    }

    @Override
    public List<Plugin> findPlugins() {
      List<Plugin> plugins = new ArrayList<>();
      for (Plugin plugin : ServiceLoader.load(Plugin.class, loader)) {
        if (plugin.getClass().getClassLoader() == loader) plugins.add(plugin);
      }
      return plugins;
    }
  }

  private static void usage(String msg) {
    System.err.println("error: " + msg);
    System.err.println("usage: vf-decompile-kt [--fixture] [--opt k=v]... [--plugins | --plugin-jar JAR...]");
    System.err.println("                       [--save legacy|folder|file|console] [--lib PATH]... [--inputs-from FILE]");
    System.err.println("                       [--print-options] INPUT... -o OUT");
    System.exit(64);
  }

  public static void main(String[] args) throws IOException {
    Map<String, Object> options = new LinkedHashMap<>();
    List<String> optPairs = new ArrayList<>();
    List<File> inputs = new ArrayList<>();
    List<File> libs = new ArrayList<>();
    List<File> pluginJars = new ArrayList<>();
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
        case "--plugin-jar":
          if (++i >= args.length) usage("--plugin-jar needs a jar");
          pluginJars.add(new File(args[i]));
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
          } else if (a.startsWith("--")) {
            usage("unknown flag " + a);
          } else {
            inputs.add(new File(a));
          }
      }
    }
    if (out == null) usage("missing -o OUT");
    if (inputs.isEmpty()) usage("no inputs");
    if (plugins && !pluginJars.isEmpty()) usage("--plugins and --plugin-jar are exclusive");
    for (File f : inputs) {
      if (!f.exists()) usage("input does not exist: " + f);
    }
    for (File f : pluginJars) {
      if (!f.isFile()) usage("plugin jar does not exist: " + f);
    }

    if (fixture) options.putAll(fixtureBase());
    for (String kv : optPairs) {
      int eq = kv.indexOf('=');
      if (eq <= 0) usage("--opt expects k=v, got " + kv);
      options.put(kv.substring(0, eq), kv.substring(eq + 1));
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

    String pluginMode;
    if (plugins) {
      pluginMode = "bundled (release jar META-INF/plugins + services)";
    } else {
      PluginSources.PLUGIN_SOURCES.clear();
      if (!pluginJars.isEmpty()) {
        PluginSources.PLUGIN_SOURCES.add(new JarServicePluginSource(pluginJars));
        pluginMode = "only " + pluginJars;
      } else {
        pluginMode = "none";
      }
    }

    if (printOptions) {
      Map<String, Object> sorted = new TreeMap<>(options);
      for (Map.Entry<String, Object> e : sorted.entrySet()) {
        Object v = e.getValue();
        System.err.println("option " + e.getKey() + "=" + (v instanceof String ? "\"" + v + "\"" : v.getClass().getName()));
      }
      System.err.println("plugins " + pluginMode + ", save " + save);
      for (PluginSource src : PluginSources.PLUGIN_SOURCES) {
        if (src instanceof JarServicePluginSource) {
          for (Plugin p : src.findPlugins()) System.err.println("plugin " + p.id() + " " + p.getClass().getName());
        }
      }
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
